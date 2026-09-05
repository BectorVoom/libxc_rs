//! GGA_C_GAPC vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_gapc.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_gapc_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = v_rho0 + v_rho1;
            let t8 = (simd::cbrt(t7));
            let t9 = f64x8::splat(1.0) / t8;
            let t10 = t6 * t9;
            let t11 = t4 * t10;
            let t13 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t11;
            let t14 = ((t11).sqrt());
            let t17 = ((t11) * (t11).sqrt());
            let t19 = t1 * t1;
            let t20 = t3 * t3;
            let t21 = t19 * t20;
            let t22 = t8 * t8;
            let t23 = f64x8::splat(1.0) / t22;
            let t25 = t21 * t5 * t23;
            let t27 = f64x8::splat(3.79785) * t14 + f64x8::splat(0.8969) * t11 + f64x8::splat(0.204775) * t17 + f64x8::splat(0.123235) * t25;
            let t30 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t27;
            let t31 = (simd::ln(t30));
            let t33 = f64x8::splat(0.062182) * t13 * t31;
            let t34 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t35 = (simd::cbrt(zeta_threshold));
            let t36 = t35 * zeta_threshold;
            let t37 = ((t34).select(t36, f64x8::splat(1.0)));
            let t40 = f64x8::splat(M_CBRT2);
            let t43 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t40 - f64x8::splat(2.0));
            let t44 = (f64x8::splat(2.0) * t37 - f64x8::splat(2.0)) * t43;
            let t46 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t51 = f64x8::splat(5.1785) * t14 + f64x8::splat(0.905775) * t11 + f64x8::splat(0.1100325) * t17 + f64x8::splat(0.1241775) * t25;
            let t54 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t51;
            let t55 = (simd::ln(t54));
            let t56 = t46 * t55;
            let t58 = f64x8::splat(0.019751789702565206) * t44 * t56;
            let t60 = f64x8::splat(1.0) / t22 / t7;
            let t61 = t6 * t60;
            let t62 = f64x8::splat(1.0) / t7;
            let t65 = f64x8::splat(1.07924) + f64x8::splat(0.03964) * t14 + f64x8::splat(0.0123825) * t11;
            let t68 = f64x8::splat(1.0) + t14 * t65 / f64x8::splat(2.0);
            let t69 = t68 * t68;
            let t70 = f64x8::splat(1.0) / t69;
            let t73 = -t33 + t58;
            let t76 = t1 * t3 * t2;
            let t78 = f64x8::splat(1.0) / t8 / t7;
            let t79 = t6 * t78;
            let t80 = t76 * t79;
            let t83 = t19 * t20 * t2;
            let t84 = t5 * t60;
            let t85 = t83 * t84;
            let t87 = t7 * t7;
            let t88 = f64x8::splat(1.0) / t87;
            let t90 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t93 = t1 * t3 / t90;
            let t95 = f64x8::splat(1.0) / t8 / t87;
            let t96 = t6 * t95;
            let t97 = t93 * t96;
            let t99 = -f64x8::splat(0.005977859662531589) * t62 + f64x8::splat(0.001317375) * t80 - f64x8::splat(0.00023775) * t85 + f64x8::splat(6.474423634745383e-06) * t88 - f64x8::splat(5.40140625e-07) * t97;
            let t101 = f64x8::splat(0.0011713266981940448) * t62 * t70 - t73 * t99;
            let t102 = f64x8::splat(1.0) / t20;
            let t103 = t1 * t102;
            let t104 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t105 = t103 * t104;
            let t106 = t14 * t11;
            let t107 = t22 * t106;
            let t108 = f64x8::splat(1.0) / t68;
            let t112 = t73 * t73;
            let t114 = f64x8::splat(0.0019711289) * t105 * t107 * t108 - f64x8::splat(2.0) * t112;
            let t115 = f64x8::splat(1.0) / t114;
            let t116 = t101 * t115;
            let t117 = t35 * t35;
            let t118 = ((t34).select(t117, f64x8::splat(1.0)));
            let t119 = t116 * t118;
            let t120 = t61 * t119;
            let t122 = f64x8::splat(1.0) + f64x8::splat(0.025) * t11;
            let t124 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t11;
            let t125 = f64x8::splat(1.0) / t124;
            let t126 = t122 * t125;
            let t128 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t129 = t126 * t128;
            let t131 = f64x8::splat(1.0) / t22 / t87;
            let t132 = t6 * t131;
            let t134 = (simd::ln(t11 / f64x8::splat(4.0)));
            let t135 = t132 * t134;
            let t136 = t118 * t118;
            let t137 = f64x8::splat(1.0) / t136;
            let t138 = t128 * t137;
            let t139 = f64x8::splat(1.0) / t122;
            let t140 = t139 * t124;
            let t141 = t138 * t140;
            let t144 = f64x8::splat(30.0) + f64x8::splat(0.0072806316506996704) * t135 * t141;
            let t145 = t128 * t95;
            let t147 = f64x8::splat(1.0) / t3;
            let t148 = t137 * t19 * t147;
            let t151 = f64x8::splat(30.0) + t145 * t148 / f64x8::splat(48.0);
            let t152 = f64x8::splat(1.0) / t151;
            let t153 = t144 * t152;
            let t154 = t103 * t6;
            let t155 = t22 * t101;
            let t159 = ((f64x8::splat(4.0)).sqrt());
            let t160 = t73 * t159;
            let t161 = t106 * t108;
            let t164 = t6 * t22;
            let t168 = f64x8::splat(0.00619125) * t160 * t161 - f64x8::splat(0.07959333333333334) * t103 * t164 * t99;
            let t169 = t168 * t115;
            let t171 = f64x8::splat(0.07959333333333334) * t154 * t155 * t115 - t169 * t73;
            let t172 = f64x8::splat(1.0) / t171;
            let t173 = t153 * t172;
            let t174 = t129 * t173;
            let t177 = -t33 + t58 + f64x8::splat(0.0010427789137624512) * t120 * t174;
            let t178 = t169 * t118;
            let t179 = t178 * t129;
            let t181 = t95 * t19 * t147;
            let t182 = t181 * t173;
            let t185 = t116 * t136;
            let t186 = t122 * t122;
            let t187 = t124 * t124;
            let t188 = f64x8::splat(1.0) / t187;
            let t189 = t186 * t188;
            let t190 = t128 * t128;
            let t191 = t189 * t190;
            let t192 = t185 * t191;
            let t193 = t87 * t87;
            let t195 = f64x8::splat(1.0) / t22 / t193;
            let t197 = t195 * t1 * t102;
            let t198 = t144 * t144;
            let t199 = t151 * t151;
            let t200 = f64x8::splat(1.0) / t199;
            let t201 = t198 * t200;
            let t202 = t171 * t171;
            let t203 = f64x8::splat(1.0) / t202;
            let t204 = t201 * t203;
            let t205 = t197 * t204;
            let t208 = f64x8::splat(1.0) + f64x8::splat(0.0013900948042322753) * t179 * t182 - f64x8::splat(5.797090694260704e-06) * t192 * t205;
            let t209 = f64x8::splat(1.0) / t208;
            let t210 = t177 * t209;
            let t211 = v_rho0 - v_rho1;
            let t212 = t211 * t62;
            let t213 = f64x8::splat(1.0) + t212;
            let t214 = (t213).simd_le(zeta_threshold);
            let t215 = (simd::cbrt(t213));
            let t217 = ((t214).select(t36, t215 * t213));
            let t218 = f64x8::splat(1.0) - t212;
            let t219 = (t218).simd_le(zeta_threshold);
            let t220 = (simd::cbrt(t218));
            let t222 = ((t219).select(t36, t220 * t218));
            let t224 = (t217 + t222 - f64x8::splat(2.0)) * t43;
            let t225 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t227 = ((t225).select(t36, f64x8::splat(2.0) * t40));
            let t228 = (f64x8::splat(0.0)).simd_le(zeta_threshold);
            let t229 = ((t228).select(t36, f64x8::splat(0.0)));
            let t231 = (t227 + t229 - f64x8::splat(2.0)) * t43;
            let t233 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t11;
            let t238 = f64x8::splat(7.05945) * t14 + f64x8::splat(1.549425) * t11 + f64x8::splat(0.420775) * t17 + f64x8::splat(0.1562925) * t25;
            let t241 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t238;
            let t242 = (simd::ln(t241));
            let t247 = t231 * (-f64x8::splat(0.03109) * t233 * t242 + t33 - f64x8::splat(0.019751789702565206) * t56);
            let t249 = f64x8::splat(0.019751789702565206) * t231 * t56;
            let t252 = f64x8::splat(1.49676) + f64x8::splat(0.00089527) * t14 + f64x8::splat(0.011799625) * t11;
            let t255 = f64x8::splat(1.0) + t14 * t252 / f64x8::splat(2.0);
            let t256 = t255 * t255;
            let t257 = f64x8::splat(1.0) / t256;
            let t260 = -t33 + t247 + t249;
            let t266 = -f64x8::splat(0.0077371026992393175) * t62 + f64x8::splat(0.00187495875) * t80 - f64x8::splat(0.000362780625) * t85 + f64x8::splat(1.0208501871552144e-05) * t88 - f64x8::splat(8.659659375e-07) * t97;
            let t268 = f64x8::splat(0.0010636476373080148) * t62 * t257 - t260 * t266;
            let t269 = f64x8::splat(1.0) / t255;
            let t273 = t260 * t260;
            let t275 = f64x8::splat(0.0005076591995833333) * t105 * t107 * t269 - f64x8::splat(2.0) * t273;
            let t276 = f64x8::splat(1.0) / t275;
            let t277 = t268 * t276;
            let t278 = t40 * t40;
            let t279 = ((t225).select(t117, t278));
            let t280 = ((t228).select(t117, f64x8::splat(0.0)));
            let t282 = t279 / f64x8::splat(2.0) + t280 / f64x8::splat(2.0);
            let t283 = t277 * t282;
            let t284 = t61 * t283;
            let t285 = t282 * t282;
            let t286 = f64x8::splat(1.0) / t285;
            let t287 = t128 * t286;
            let t288 = t287 * t140;
            let t291 = f64x8::splat(30.0) + f64x8::splat(0.0036401987395106744) * t135 * t288;
            let t293 = t286 * t19 * t147;
            let t296 = f64x8::splat(30.0) + t145 * t293 / f64x8::splat(48.0);
            let t297 = f64x8::splat(1.0) / t296;
            let t298 = t291 * t297;
            let t299 = t22 * t268;
            let t303 = t260 * t159;
            let t304 = t106 * t269;
            let t310 = f64x8::splat(0.0058998125) * t303 * t304 - f64x8::splat(0.021511666666666665) * t103 * t164 * t266;
            let t311 = t310 * t276;
            let t313 = f64x8::splat(0.021511666666666665) * t154 * t299 * t276 - t311 * t260;
            let t314 = f64x8::splat(1.0) / t313;
            let t315 = t298 * t314;
            let t316 = t129 * t315;
            let t319 = -t33 + t247 + t249 + f64x8::splat(0.000281831548704497) * t284 * t316;
            let t320 = t311 * t282;
            let t321 = t320 * t129;
            let t322 = t181 * t315;
            let t325 = t277 * t285;
            let t326 = t325 * t191;
            let t327 = t291 * t291;
            let t328 = t296 * t296;
            let t329 = f64x8::splat(1.0) / t328;
            let t330 = t327 * t329;
            let t331 = t313 * t313;
            let t332 = f64x8::splat(1.0) / t331;
            let t333 = t330 * t332;
            let t334 = t197 * t333;
            let t337 = f64x8::splat(1.0) + f64x8::splat(0.0013900948042322753) * t321 * t322 - f64x8::splat(5.797090694260704e-06) * t326 * t334;
            let t338 = f64x8::splat(1.0) / t337;
            let t340 = t319 * t338 - t210;
            let t341 = t224 * t340;
            let tzk0 = t210 + t341;
            acc_zk = tzk0;
            let t344 = f64x8::splat(0.0011073577833333333) * t4 * t79 * t31;
            let t345 = t27 * t27;
            let t346 = f64x8::splat(1.0) / t345;
            let t347 = t13 * t346;
            let t348 = f64x8::splat(1.0) / t14;
            let t349 = t348 * t1;
            let t350 = t3 * t6;
            let t351 = t350 * t78;
            let t352 = t349 * t351;
            let t354 = t4 * t79;
            let t356 = ((t11).sqrt());
            let t357 = t356 * t1;
            let t358 = t357 * t351;
            let t360 = t21 * t84;
            let t362 = -f64x8::splat(0.632975) * t352 - f64x8::splat(0.29896666666666666) * t354 - f64x8::splat(0.1023875) * t358 - f64x8::splat(0.08215666666666667) * t360;
            let t363 = f64x8::splat(1.0) / t30;
            let t364 = t362 * t363;
            let t366 = f64x8::splat(1.0) * t347 * t364;
            let t367 = t44 * t1;
            let t369 = t350 * t78 * t55;
            let t371 = f64x8::splat(0.0001831155503675316) * t367 * t369;
            let t372 = t44 * t46;
            let t373 = t51 * t51;
            let t374 = f64x8::splat(1.0) / t373;
            let t379 = -f64x8::splat(0.8630833333333333) * t352 - f64x8::splat(0.301925) * t354 - f64x8::splat(0.05501625) * t358 - f64x8::splat(0.082785) * t360;
            let t381 = f64x8::splat(1.0) / t54;
            let t382 = t374 * t379 * t381;
            let t384 = f64x8::splat(0.5848223397455204) * t372 * t382;
            let t385 = t132 * t119;
            let t391 = f64x8::splat(1.0) / t69 / t68;
            let t392 = t62 * t391;
            let t394 = t348 * t65 * t1;
            let t399 = -f64x8::splat(0.006606666666666667) * t352 - f64x8::splat(0.0041275) * t354;
            let t402 = -t394 * t351 / f64x8::splat(12.0) + t14 * t399 / f64x8::splat(2.0);
            let t405 = t344 + t366 - t371 - t384;
            let t408 = t76 * t96;
            let t410 = t5 * t131;
            let t411 = t83 * t410;
            let t413 = t87 * t7;
            let t414 = f64x8::splat(1.0) / t413;
            let t417 = f64x8::splat(1.0) / t8 / t413;
            let t418 = t6 * t417;
            let t419 = t93 * t418;
            let t421 = f64x8::splat(0.005977859662531589) * t88 - f64x8::splat(0.0017565) * t408 + f64x8::splat(0.00039625) * t411 - f64x8::splat(1.2948847269490767e-05) * t414 + f64x8::splat(1.260328125e-06) * t419;
            let t423 = -f64x8::splat(0.0011713266981940448) * t88 * t70 - f64x8::splat(0.0023426533963880895) * t392 * t402 - t405 * t99 - t73 * t421;
            let t424 = t423 * t115;
            let t425 = t424 * t118;
            let t426 = t61 * t425;
            let t429 = t61 * t101;
            let t430 = t114 * t114;
            let t431 = f64x8::splat(1.0) / t430;
            let t432 = t431 * t118;
            let t433 = t432 * t122;
            let t434 = t429 * t433;
            let t435 = t125 * t128;
            let t436 = t435 * t144;
            let t437 = t152 * t172;
            let t438 = t9 * t106;
            let t442 = t19 * t147;
            let t443 = t104 * t104;
            let t444 = t443 * t443;
            let t445 = t444 * t104;
            let t446 = t442 * t445;
            let t447 = t23 * t14;
            let t451 = t70 * t402;
            let t457 = f64x8::splat(0.0013140859333333334) * t105 * t438 * t108 - f64x8::splat(0.00098556445) * t446 * t447 * t108 - f64x8::splat(0.0019711289) * t105 * t107 * t451 - f64x8::splat(4.0) * t73 * t405;
            let t458 = t437 * t457;
            let t459 = t436 * t458;
            let t462 = t5 * t414;
            let t463 = t462 * t101;
            let t464 = t115 * t118;
            let t465 = t464 * t1;
            let t466 = t463 * t465;
            let t467 = t3 * t125;
            let t468 = t467 * t128;
            let t469 = t468 * t173;
            let t472 = t464 * t122;
            let t473 = t463 * t472;
            let t474 = t188 * t128;
            let t475 = t474 * t144;
            let t476 = t437 * t4;
            let t477 = t475 * t476;
            let t481 = f64x8::splat(1.0) / t22 / t413;
            let t482 = t6 * t481;
            let t483 = t482 * t134;
            let t486 = t482 * t128;
            let t487 = t137 * t139;
            let t488 = t487 * t124;
            let t491 = f64x8::splat(1.0) / t193;
            let t492 = t5 * t491;
            let t493 = t134 * t128;
            let t494 = t492 * t493;
            let t495 = f64x8::splat(1.0) / t186;
            let t496 = t137 * t495;
            let t498 = t124 * t1 * t3;
            let t499 = t496 * t498;
            let t502 = t487 * t4;
            let t505 = -f64x8::splat(0.01941501773519912) * t483 * t141 - f64x8::splat(0.00242687721689989) * t486 * t488 + f64x8::splat(0.00024268772168998902) * t494 * t499 - f64x8::splat(0.0004314987691648005) * t494 * t502;
            let t506 = t505 * t152;
            let t507 = t506 * t172;
            let t508 = t129 * t507;
            let t511 = t193 * t7;
            let t512 = f64x8::splat(1.0) / t511;
            let t513 = t6 * t512;
            let t514 = t513 * t101;
            let t515 = f64x8::splat(1.0) / t118;
            let t516 = t115 * t515;
            let t517 = t516 * t122;
            let t518 = t514 * t517;
            let t519 = t125 * t190;
            let t520 = t519 * t144;
            let t521 = t200 * t172;
            let t522 = t521 * t442;
            let t523 = t520 * t522;
            let t526 = t429 * t472;
            let t527 = t152 * t203;
            let t528 = t9 * t101;
            let t532 = t22 * t423;
            let t536 = t431 * t457;
            let t540 = t405 * t159;
            let t543 = t73 * t104;
            let t544 = t543 * t14;
            let t545 = t108 * t1;
            let t546 = t3 * t78;
            let t547 = t545 * t546;
            let t550 = t106 * t70;
            let t551 = t550 * t402;
            let t560 = f64x8::splat(0.00619125) * t540 * t161 - f64x8::splat(0.0123825) * t544 * t547 - f64x8::splat(0.00619125) * t160 * t551 - f64x8::splat(0.05306222222222222) * t103 * t10 * t99 - f64x8::splat(0.07959333333333334) * t103 * t164 * t421;
            let t561 = t560 * t115;
            let t563 = t168 * t431;
            let t564 = t73 * t457;
            let t567 = f64x8::splat(0.05306222222222222) * t154 * t528 * t115 + f64x8::splat(0.07959333333333334) * t154 * t532 * t115 - f64x8::splat(0.07959333333333334) * t154 * t155 * t536 - t561 * t73 + t563 * t564 - t169 * t405;
            let t568 = t527 * t567;
            let t569 = t436 * t568;
            let t572 = t344 + t366 - t371 - t384 - f64x8::splat(0.001737964856270752) * t385 * t174 + f64x8::splat(0.0010427789137624512) * t426 * t174 - f64x8::splat(0.0010427789137624512) * t434 * t459 - f64x8::splat(3.475929712541504e-05) * t466 * t469 + f64x8::splat(6.180203028898794e-05) * t473 * t477 + f64x8::splat(0.0010427789137624512) * t120 * t508 + f64x8::splat(5.069064164123027e-05) * t518 * t523 - f64x8::splat(0.0010427789137624512) * t526 * t569;
            let t573 = t572 * t209;
            let t574 = t208 * t208;
            let t575 = f64x8::splat(1.0) / t574;
            let t576 = t177 * t575;
            let t577 = t561 * t118;
            let t578 = t577 * t129;
            let t581 = t563 * t118;
            let t582 = t581 * t129;
            let t583 = t172 * t457;
            let t584 = t153 * t583;
            let t585 = t181 * t584;
            let t588 = t118 * t6;
            let t589 = t588 * t481;
            let t590 = t169 * t589;
            let t591 = t435 * t173;
            let t594 = t118 * t122;
            let t595 = t594 * t188;
            let t596 = t169 * t595;
            let t597 = t128 * t481;
            let t599 = t437 * t6;
            let t600 = t597 * t144 * t599;
            let t604 = t417 * t19 * t147;
            let t605 = t604 * t173;
            let t608 = t181 * t507;
            let t611 = t169 * t515;
            let t612 = t126 * t190;
            let t613 = t611 * t612;
            let t615 = f64x8::splat(1.0) / t22 / t511;
            let t616 = t615 * t1;
            let t617 = t616 * t102;
            let t618 = t144 * t200;
            let t619 = t618 * t172;
            let t620 = t617 * t619;
            let t623 = t203 * t567;
            let t624 = t153 * t623;
            let t625 = t181 * t624;
            let t628 = t424 * t136;
            let t629 = t628 * t191;
            let t632 = t101 * t431;
            let t633 = t632 * t136;
            let t634 = t633 * t191;
            let t635 = t203 * t457;
            let t636 = t201 * t635;
            let t637 = t197 * t636;
            let t640 = t122 * t188;
            let t641 = t640 * t190;
            let t642 = t185 * t641;
            let t643 = t193 * t87;
            let t644 = f64x8::splat(1.0) / t643;
            let t645 = t644 * t19;
            let t646 = t645 * t147;
            let t647 = t203 * t6;
            let t648 = t201 * t647;
            let t649 = t646 * t648;
            let t653 = f64x8::splat(1.0) / t187 / t124;
            let t654 = t186 * t653;
            let t655 = t654 * t190;
            let t656 = t185 * t655;
            let t659 = t617 * t204;
            let t662 = t203 * t505;
            let t663 = t618 * t662;
            let t664 = t197 * t663;
            let t667 = t190 * t128;
            let t668 = t189 * t667;
            let t669 = t116 * t668;
            let t670 = t193 * t193;
            let t671 = f64x8::splat(1.0) / t670;
            let t672 = t671 * f64x8::splat(M_PI);
            let t674 = f64x8::splat(1.0) / t199 / t151;
            let t675 = t198 * t674;
            let t676 = t675 * t203;
            let t677 = t672 * t676;
            let t681 = f64x8::splat(1.0) / t202 / t171;
            let t682 = t681 * t567;
            let t683 = t201 * t682;
            let t684 = t197 * t683;
            let t687 = f64x8::splat(0.0013900948042322753) * t578 * t182 - f64x8::splat(0.0013900948042322753) * t582 * t585 - f64x8::splat(3.4752370105806884e-05) * t590 * t591 + f64x8::splat(6.178971404812464e-05) * t596 * t600 - f64x8::splat(0.0032435545432086426) * t179 * t605 + f64x8::splat(0.0013900948042322753) * t179 * t608 + f64x8::splat(0.00020272215895054016) * t613 * t620 - f64x8::splat(0.0013900948042322753) * t179 * t625 - f64x8::splat(5.797090694260704e-06) * t629 * t205 + f64x8::splat(5.797090694260704e-06) * t634 * t637 + f64x8::splat(9.661817823767841e-08) * t642 * t649 - f64x8::splat(1.7178712090659221e-07) * t656 * t649 + f64x8::splat(2.7053089906549955e-05) * t192 * t659 - f64x8::splat(1.1594181388521409e-05) * t192 * t664 - f64x8::splat(1.6908181191593722e-06) * t669 * t677 + f64x8::splat(1.1594181388521409e-05) * t192 * t684;
            let t688 = t576 * t687;
            let t689 = t211 * t88;
            let t690 = t62 - t689;
            let t693 = ((t214).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t215 * t690));
            let t694 = -t690;
            let t697 = ((t219).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t220 * t694));
            let t699 = (t693 + t697) * t43;
            let t700 = t699 * t340;
            let t704 = t238 * t238;
            let t705 = f64x8::splat(1.0) / t704;
            let t706 = t233 * t705;
            let t711 = -f64x8::splat(1.176575) * t352 - f64x8::splat(0.516475) * t354 - f64x8::splat(0.2103875) * t358 - f64x8::splat(0.104195) * t360;
            let t712 = f64x8::splat(1.0) / t241;
            let t713 = t711 * t712;
            let t719 = t46 * t374;
            let t720 = t379 * t381;
            let t724 = t231 * (f64x8::splat(0.0005323644333333333) * t4 * t79 * t242 + f64x8::splat(1.0) * t706 * t713 - t344 - t366 + f64x8::splat(0.0001831155503675316) * t4 * t79 * t55 + f64x8::splat(0.5848223397455204) * t719 * t720);
            let t725 = t231 * t1;
            let t727 = f64x8::splat(0.0001831155503675316) * t725 * t369;
            let t728 = t231 * t46;
            let t730 = f64x8::splat(0.5848223397455204) * t728 * t382;
            let t731 = t132 * t283;
            let t737 = f64x8::splat(1.0) / t256 / t255;
            let t738 = t62 * t737;
            let t740 = t348 * t252 * t1;
            let t745 = -f64x8::splat(0.00014921166666666667) * t352 - f64x8::splat(0.003933208333333334) * t354;
            let t748 = -t740 * t351 / f64x8::splat(12.0) + t14 * t745 / f64x8::splat(2.0);
            let t751 = t344 + t366 + t724 - t727 - t730;
            let t758 = f64x8::splat(0.0077371026992393175) * t88 - f64x8::splat(0.002499945) * t408 + f64x8::splat(0.000604634375) * t411 - f64x8::splat(2.0417003743104288e-05) * t414 + f64x8::splat(2.0205871875e-06) * t419;
            let t760 = -f64x8::splat(0.0010636476373080148) * t88 * t257 - f64x8::splat(0.0021272952746160295) * t738 * t748 - t751 * t266 - t260 * t758;
            let t761 = t760 * t276;
            let t762 = t761 * t282;
            let t763 = t61 * t762;
            let t766 = t61 * t268;
            let t767 = t275 * t275;
            let t768 = f64x8::splat(1.0) / t767;
            let t769 = t768 * t282;
            let t770 = t769 * t122;
            let t771 = t766 * t770;
            let t772 = t435 * t291;
            let t773 = t297 * t314;
            let t780 = t257 * t748;
            let t786 = f64x8::splat(0.0003384394663888889) * t105 * t438 * t269 - f64x8::splat(0.00025382959979166665) * t446 * t447 * t269 - f64x8::splat(0.0005076591995833333) * t105 * t107 * t780 - f64x8::splat(4.0) * t260 * t751;
            let t787 = t773 * t786;
            let t788 = t772 * t787;
            let t791 = t462 * t268;
            let t792 = t276 * t282;
            let t793 = t792 * t1;
            let t794 = t791 * t793;
            let t795 = t468 * t315;
            let t798 = t792 * t122;
            let t799 = t791 * t798;
            let t800 = t474 * t291;
            let t801 = t773 * t4;
            let t802 = t800 * t801;
            let t807 = t286 * t139;
            let t808 = t807 * t124;
            let t811 = t286 * t495;
            let t812 = t811 * t498;
            let t815 = t807 * t4;
            let t818 = -f64x8::splat(0.009707196638695131) * t483 * t288 - f64x8::splat(0.0012133995798368914) * t486 * t808 + f64x8::splat(0.00012133995798368915) * t494 * t812 - f64x8::splat(0.0002157424452949993) * t494 * t815;
            let t819 = t818 * t297;
            let t820 = t819 * t314;
            let t821 = t129 * t820;
            let t824 = t513 * t268;
            let t825 = f64x8::splat(1.0) / t282;
            let t826 = t276 * t825;
            let t827 = t826 * t122;
            let t828 = t824 * t827;
            let t829 = t519 * t291;
            let t830 = t329 * t314;
            let t831 = t830 * t442;
            let t832 = t829 * t831;
            let t835 = t766 * t798;
            let t836 = t297 * t332;
            let t837 = t9 * t268;
            let t841 = t22 * t760;
            let t845 = t768 * t786;
            let t849 = t751 * t159;
            let t852 = t260 * t104;
            let t853 = t852 * t14;
            let t854 = t269 * t1;
            let t855 = t854 * t546;
            let t858 = t106 * t257;
            let t859 = t858 * t748;
            let t868 = f64x8::splat(0.0058998125) * t849 * t304 - f64x8::splat(0.011799625) * t853 * t855 - f64x8::splat(0.0058998125) * t303 * t859 - f64x8::splat(0.014341111111111112) * t103 * t10 * t266 - f64x8::splat(0.021511666666666665) * t103 * t164 * t758;
            let t869 = t868 * t276;
            let t871 = t310 * t768;
            let t872 = t260 * t786;
            let t875 = f64x8::splat(0.014341111111111112) * t154 * t837 * t276 + f64x8::splat(0.021511666666666665) * t154 * t841 * t276 - f64x8::splat(0.021511666666666665) * t154 * t299 * t845 - t869 * t260 + t871 * t872 - t311 * t751;
            let t876 = t836 * t875;
            let t877 = t772 * t876;
            let t880 = t344 + t366 + t724 - t727 - t730 - f64x8::splat(0.0004697192478408283) * t731 * t316 + f64x8::splat(0.000281831548704497) * t763 * t316 - f64x8::splat(0.000281831548704497) * t771 * t788 - f64x8::splat(9.394384956816566e-06) * t794 * t795 + f64x8::splat(1.6703216453219856e-05) * t799 * t802 + f64x8::splat(0.000281831548704497) * t284 * t821 + f64x8::splat(1.3700144728690826e-05) * t828 * t832 - f64x8::splat(0.000281831548704497) * t835 * t877;
            let t882 = t337 * t337;
            let t883 = f64x8::splat(1.0) / t882;
            let t884 = t319 * t883;
            let t885 = t869 * t282;
            let t886 = t885 * t129;
            let t889 = t871 * t282;
            let t890 = t889 * t129;
            let t891 = t314 * t786;
            let t892 = t298 * t891;
            let t893 = t181 * t892;
            let t896 = t282 * t6;
            let t897 = t896 * t481;
            let t898 = t311 * t897;
            let t899 = t435 * t315;
            let t902 = t282 * t122;
            let t903 = t902 * t188;
            let t904 = t311 * t903;
            let t906 = t773 * t6;
            let t907 = t597 * t291 * t906;
            let t910 = t604 * t315;
            let t913 = t181 * t820;
            let t916 = t311 * t825;
            let t917 = t916 * t612;
            let t918 = t291 * t329;
            let t919 = t918 * t314;
            let t920 = t617 * t919;
            let t923 = t332 * t875;
            let t924 = t298 * t923;
            let t925 = t181 * t924;
            let t928 = t761 * t285;
            let t929 = t928 * t191;
            let t932 = t268 * t768;
            let t933 = t932 * t285;
            let t934 = t933 * t191;
            let t935 = t332 * t786;
            let t936 = t330 * t935;
            let t937 = t197 * t936;
            let t940 = t325 * t641;
            let t941 = t332 * t6;
            let t942 = t330 * t941;
            let t943 = t646 * t942;
            let t946 = t325 * t655;
            let t949 = t617 * t333;
            let t952 = t332 * t818;
            let t953 = t918 * t952;
            let t954 = t197 * t953;
            let t957 = t277 * t668;
            let t959 = f64x8::splat(1.0) / t328 / t296;
            let t960 = t327 * t959;
            let t961 = t960 * t332;
            let t962 = t672 * t961;
            let t966 = f64x8::splat(1.0) / t331 / t313;
            let t967 = t966 * t875;
            let t968 = t330 * t967;
            let t969 = t197 * t968;
            let t972 = f64x8::splat(0.0013900948042322753) * t886 * t322 - f64x8::splat(0.0013900948042322753) * t890 * t893 - f64x8::splat(3.4752370105806884e-05) * t898 * t899 + f64x8::splat(6.178971404812464e-05) * t904 * t907 - f64x8::splat(0.0032435545432086426) * t321 * t910 + f64x8::splat(0.0013900948042322753) * t321 * t913 + f64x8::splat(0.00020272215895054016) * t917 * t920 - f64x8::splat(0.0013900948042322753) * t321 * t925 - f64x8::splat(5.797090694260704e-06) * t929 * t334 + f64x8::splat(5.797090694260704e-06) * t934 * t937 + f64x8::splat(9.661817823767841e-08) * t940 * t943 - f64x8::splat(1.7178712090659221e-07) * t946 * t943 + f64x8::splat(2.7053089906549955e-05) * t326 * t949 - f64x8::splat(1.1594181388521409e-05) * t326 * t954 - f64x8::splat(1.6908181191593722e-06) * t957 * t962 + f64x8::splat(1.1594181388521409e-05) * t326 * t969;
            let t974 = t338 * t880 - t884 * t972 - t573 + t688;
            let t975 = t224 * t974;
            let tvrho0 = t210 + t341 + t7 * (t573 - t688 + t700 + t975);
            acc_vrho_0 = tvrho0;
            let t978 = -t62 - t689;
            let t981 = ((t214).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t215 * t978));
            let t982 = -t978;
            let t985 = ((t219).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t220 * t982));
            let t987 = (t981 + t985) * t43;
            let t988 = t987 * t340;
            let tvrho1 = t210 + t341 + t7 * (t573 - t688 + t988 + t975);
            acc_vrho_1 = tvrho1;
            let t991 = t126 * t173;
            let t992 = t120 * t991;
            let t995 = f64x8::splat(1.0) / t8 / t193;
            let t996 = t5 * t995;
            let t997 = t996 * t116;
            let t998 = t515 * t128;
            let t999 = t134 * t152;
            let t1000 = t999 * t172;
            let t1001 = t998 * t1000;
            let t1002 = t997 * t1001;
            let t1004 = t6 * t491;
            let t1005 = t1004 * t101;
            let t1006 = t1005 * t517;
            let t1007 = t436 * t522;
            let t1008 = t1006 * t1007;
            let t1010 = f64x8::splat(0.0010427789137624512) * t992 + f64x8::splat(3.0368356656884498e-05) * t1002 - f64x8::splat(2.1724560703384402e-05) * t1008;
            let t1011 = t1010 * t209;
            let t1012 = t594 * t125;
            let t1013 = t169 * t1012;
            let t1014 = t1013 * t182;
            let t1016 = t998 * t512;
            let t1018 = t442 * t6;
            let t1019 = t1018 * t1000;
            let t1020 = t169 * t1016 * t1019;
            let t1022 = t611 * t129;
            let t1023 = t197 * t619;
            let t1024 = t1022 * t1023;
            let t1026 = t189 * t128;
            let t1027 = t185 * t1026;
            let t1028 = t1027 * t205;
            let t1030 = t116 * t122;
            let t1031 = t193 * t413;
            let t1033 = f64x8::splat(1.0) / t8 / t1031;
            let t1034 = t519 * t1033;
            let t1035 = t1030 * t1034;
            let t1036 = t103 * t144;
            let t1037 = t200 * t203;
            let t1038 = t6 * t134;
            let t1039 = t1037 * t1038;
            let t1040 = t1036 * t1039;
            let t1041 = t1035 * t1040;
            let t1043 = t116 * t191;
            let t1044 = f64x8::splat(1.0) / t1031;
            let t1045 = t1044 * f64x8::splat(M_PI);
            let t1046 = t1045 * t676;
            let t1047 = t1043 * t1046;
            let t1049 = f64x8::splat(0.0013900948042322753) * t1014 + f64x8::splat(1.0120768229166667e-05) * t1020 - f64x8::splat(8.688092526451721e-05) * t1024 - f64x8::splat(1.1594181388521409e-05) * t1028 - f64x8::splat(8.441296398122203e-08) * t1041 + f64x8::splat(7.24636336782588e-07) * t1047;
            let t1050 = t576 * t1049;
            let t1051 = t126 * t315;
            let t1052 = t284 * t1051;
            let t1054 = t996 * t277;
            let t1055 = t825 * t128;
            let t1056 = t134 * t297;
            let t1057 = t1056 * t314;
            let t1058 = t1055 * t1057;
            let t1059 = t1054 * t1058;
            let t1061 = t1004 * t268;
            let t1062 = t1061 * t827;
            let t1063 = t772 * t831;
            let t1064 = t1062 * t1063;
            let t1066 = f64x8::splat(0.000281831548704497) * t1052 + f64x8::splat(4.103691393393805e-06) * t1059 - f64x8::splat(5.871490598010354e-06) * t1064;
            let t1068 = t902 * t125;
            let t1069 = t311 * t1068;
            let t1070 = t1069 * t322;
            let t1072 = t1055 * t512;
            let t1074 = t1018 * t1057;
            let t1075 = t311 * t1072 * t1074;
            let t1077 = t916 * t129;
            let t1078 = t197 * t919;
            let t1079 = t1077 * t1078;
            let t1081 = t325 * t1026;
            let t1082 = t1081 * t334;
            let t1084 = t277 * t122;
            let t1085 = t1084 * t1034;
            let t1086 = t103 * t291;
            let t1087 = t329 * t332;
            let t1088 = t1087 * t1038;
            let t1089 = t1086 * t1088;
            let t1090 = t1085 * t1089;
            let t1092 = t277 * t191;
            let t1093 = t1045 * t961;
            let t1094 = t1092 * t1093;
            let t1096 = f64x8::splat(0.0013900948042322753) * t1070 + f64x8::splat(5.060221354166667e-06) * t1075 - f64x8::splat(8.688092526451721e-05) * t1079 - f64x8::splat(1.1594181388521409e-05) * t1082 - f64x8::splat(4.2205124476153754e-08) * t1090 + f64x8::splat(7.24636336782588e-07) * t1094;
            let t1098 = t1066 * t338 - t1096 * t884 - t1011 + t1050;
            let t1099 = t224 * t1098;
            let tvsigma0 = t7 * (t1011 - t1050 + t1099);
            acc_vsigma_0 = tvsigma0;
            let t1104 = f64x8::splat(0.0020855578275249024) * t992 + f64x8::splat(6.0736713313768996e-05) * t1002 - f64x8::splat(4.3449121406768804e-05) * t1008;
            let t1105 = t1104 * t209;
            let t1112 = f64x8::splat(0.0027801896084645506) * t1014 + f64x8::splat(2.0241536458333334e-05) * t1020 - f64x8::splat(0.00017376185052903441) * t1024 - f64x8::splat(2.3188362777042817e-05) * t1028 - f64x8::splat(1.6882592796244405e-07) * t1041 + f64x8::splat(1.449272673565176e-06) * t1047;
            let t1113 = t576 * t1112;
            let t1117 = f64x8::splat(0.000563663097408994) * t1052 + f64x8::splat(8.20738278678761e-06) * t1059 - f64x8::splat(1.1742981196020707e-05) * t1064;
            let t1125 = f64x8::splat(0.0027801896084645506) * t1070 + f64x8::splat(1.0120442708333334e-05) * t1075 - f64x8::splat(0.00017376185052903441) * t1079 - f64x8::splat(2.3188362777042817e-05) * t1082 - f64x8::splat(8.441024895230751e-08) * t1090 + f64x8::splat(1.449272673565176e-06) * t1094;
            let t1127 = t1117 * t338 - t1125 * t884 - t1105 + t1113;
            let t1128 = t224 * t1127;
            let tvsigma1 = t7 * (t1105 - t1113 + t1128);
            acc_vsigma_1 = tvsigma1;
            let tvsigma2 = tvsigma0;
            acc_vsigma_2 = tvsigma2;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
