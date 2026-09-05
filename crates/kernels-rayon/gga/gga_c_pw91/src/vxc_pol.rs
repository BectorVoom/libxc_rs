//! GGA_C_PW91 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pw91.c`
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
pub fn gga_c_pw91_vxc_pol(
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
            let t11 = t4 * t6 / t8;
            let t13 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t11;
            let t14 = ((t11).sqrt());
            let t17 = ((t11) * (t11).sqrt());
            let t19 = t1 * t1;
            let t20 = t3 * t3;
            let t21 = t19 * t20;
            let t22 = t8 * t8;
            let t25 = t21 * t5 / t22;
            let t27 = f64x8::splat(3.79785) * t14 + f64x8::splat(0.8969) * t11 + f64x8::splat(0.204775) * t17 + f64x8::splat(0.123235) * t25;
            let t30 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t27;
            let t31 = (simd::ln(t30));
            let t33 = f64x8::splat(0.062182) * t13 * t31;
            let t34 = v_rho0 - v_rho1;
            let t35 = t34 * t34;
            let t36 = t35 * t35;
            let t37 = t7 * t7;
            let t38 = t37 * t37;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t36 * t39;
            let t41 = f64x8::splat(1.0) / t7;
            let t42 = t34 * t41;
            let t43 = f64x8::splat(1.0) + t42;
            let t44 = (t43).simd_le(zeta_threshold);
            let t45 = (simd::cbrt(zeta_threshold));
            let t46 = t45 * zeta_threshold;
            let t47 = (simd::cbrt(t43));
            let t48 = t47 * t43;
            let t49 = ((t44).select(t46, t48));
            let t50 = f64x8::splat(1.0) - t42;
            let t51 = (t50).simd_le(zeta_threshold);
            let t52 = (simd::cbrt(t50));
            let t53 = t52 * t50;
            let t54 = ((t51).select(t46, t53));
            let t55 = t49 + t54 - f64x8::splat(2.0);
            let t56 = f64x8::splat(M_CBRT2);
            let t59 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t56 - f64x8::splat(2.0));
            let t60 = t55 * t59;
            let t62 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t11;
            let t67 = f64x8::splat(7.05945) * t14 + f64x8::splat(1.549425) * t11 + f64x8::splat(0.420775) * t17 + f64x8::splat(0.1562925) * t25;
            let t70 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t67;
            let t71 = (simd::ln(t70));
            let t75 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t80 = f64x8::splat(5.1785) * t14 + f64x8::splat(0.905775) * t11 + f64x8::splat(0.1100325) * t17 + f64x8::splat(0.1241775) * t25;
            let t83 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t80;
            let t84 = (simd::ln(t83));
            let t85 = t75 * t84;
            let t87 = -f64x8::splat(0.03109) * t62 * t71 + t33 - f64x8::splat(0.019751789702565206) * t85;
            let t88 = t60 * t87;
            let t89 = t40 * t88;
            let t91 = f64x8::splat(0.019751789702565206) * t60 * t85;
            let t92 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t93 = (simd::cbrt(t92));
            let t94 = t93 * t93;
            let t95 = t19 * t94;
            let t96 = t45 * t45;
            let t97 = t47 * t47;
            let t98 = ((t44).select(t96, t97));
            let t99 = t52 * t52;
            let t100 = ((t51).select(t96, t99));
            let t102 = t98 / f64x8::splat(2.0) + t100 / f64x8::splat(2.0);
            let t103 = t102 * t102;
            let t104 = t103 * t102;
            let t105 = f64x8::splat(1.0) / t93;
            let t106 = t19 * t105;
            let t108 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t110 = f64x8::splat(1.0) / t8 / t37;
            let t111 = t108 * t110;
            let t112 = t111 * t56;
            let t113 = f64x8::splat(1.0) / t103;
            let t115 = f64x8::splat(1.0) / t3;
            let t116 = t115 * t5;
            let t117 = t113 * t19 * t116;
            let t120 = -t33 + t89 + t91;
            let t121 = f64x8::splat(1.0) / t104;
            let t123 = f64x8::splat(1.0) / t94;
            let t124 = t1 * t123;
            let t127 = (simd::exp(-f64x8::splat(128.97460341341235) * t120 * t121 * t124));
            let t128 = t127 - f64x8::splat(1.0);
            let t129 = f64x8::splat(1.0) / t128;
            let t130 = t105 * t129;
            let t131 = t108 * t108;
            let t133 = f64x8::splat(1.0) / t22 / t38;
            let t134 = t131 * t133;
            let t135 = t130 * t134;
            let t136 = t56 * t56;
            let t137 = t103 * t103;
            let t138 = f64x8::splat(1.0) / t137;
            let t139 = t136 * t138;
            let t140 = f64x8::splat(1.0) / t20;
            let t141 = t140 * t6;
            let t142 = t139 * t141;
            let t145 = t112 * t117 / f64x8::splat(96.0) + f64x8::splat(0.0027166129655589867) * t135 * t142;
            let t146 = t1 * t105;
            let t147 = t129 * t108;
            let t148 = t146 * t147;
            let t149 = t110 * t56;
            let t150 = t113 * t115;
            let t151 = t150 * t5;
            let t155 = t19 * t123;
            let t156 = t128 * t128;
            let t157 = f64x8::splat(1.0) / t156;
            let t158 = t157 * t131;
            let t159 = t155 * t158;
            let t160 = t133 * t136;
            let t161 = t138 * t140;
            let t162 = t161 * t6;
            let t163 = t160 * t162;
            let t166 = f64x8::splat(1.0) + f64x8::splat(0.08693161489788757) * t148 * t149 * t151 + f64x8::splat(0.0075571056687546295) * t159 * t163;
            let t167 = f64x8::splat(1.0) / t166;
            let t171 = f64x8::splat(1.0) + f64x8::splat(2.7818116767324024) * t106 * t145 * t167;
            let t172 = (simd::ln(t171));
            let t175 = f64x8::splat(0.002584488143490343) * t95 * t104 * t172;
            let t176 = t2 * t93;
            let t179 = f64x8::splat(2.568) + f64x8::splat(5.8165) * t11 + f64x8::splat(0.00184725) * t25;
            let t182 = f64x8::splat(1000.0) + f64x8::splat(2180.75) * t11 + f64x8::splat(118.0) * t25;
            let t183 = f64x8::splat(1.0) / t182;
            let t185 = t179 * t183 - f64x8::splat(0.0018535714285714286);
            let t186 = t185 * t102;
            let t187 = t186 * t108;
            let t188 = t176 * t187;
            let t190 = (simd::cbrt(f64x8::splat(9.0)));
            let t191 = t190 * t190;
            let t193 = t2 * t5 * t191 * t3;
            let t195 = f64x8::splat(1.0) / t22 / t37;
            let t197 = t108 * t56;
            let t201 = (simd::exp(-f64x8::splat(25.0) / f64x8::splat(18.0) * t193 * t195 * t103 * t197));
            let t202 = t116 * t201;
            let t203 = t149 * t202;
            let t205 = t188 * t203 / f64x8::splat(2.0);
            let tzk0 = -t33 + t89 + t91 + t175 + t205;
            acc_zk = tzk0;
            let t207 = f64x8::splat(1.0) / t8 / t7;
            let t208 = t6 * t207;
            let t210 = t4 * t208 * t31;
            let t211 = f64x8::splat(0.0011073577833333333) * t210;
            let t212 = t27 * t27;
            let t213 = f64x8::splat(1.0) / t212;
            let t214 = t13 * t213;
            let t216 = f64x8::splat(1.0) / t14 * t1;
            let t217 = t3 * t6;
            let t218 = t217 * t207;
            let t219 = t216 * t218;
            let t221 = t4 * t208;
            let t223 = ((t11).sqrt());
            let t224 = t223 * t1;
            let t225 = t224 * t218;
            let t230 = t21 * t5 / t22 / t7;
            let t232 = -f64x8::splat(0.632975) * t219 - f64x8::splat(0.29896666666666666) * t221 - f64x8::splat(0.1023875) * t225 - f64x8::splat(0.08215666666666667) * t230;
            let t233 = f64x8::splat(1.0) / t30;
            let t234 = t232 * t233;
            let t235 = t214 * t234;
            let t236 = f64x8::splat(1.0) * t235;
            let t237 = t35 * t34;
            let t238 = t237 * t39;
            let t239 = t238 * t88;
            let t240 = f64x8::splat(4.0) * t239;
            let t241 = t38 * t7;
            let t242 = f64x8::splat(1.0) / t241;
            let t243 = t36 * t242;
            let t244 = t243 * t88;
            let t245 = f64x8::splat(4.0) * t244;
            let t246 = f64x8::splat(1.0) / t37;
            let t247 = t34 * t246;
            let t248 = t41 - t247;
            let t251 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t248));
            let t252 = -t248;
            let t255 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t252));
            let t257 = (t251 + t255) * t59;
            let t258 = t257 * t87;
            let t259 = t40 * t258;
            let t263 = t67 * t67;
            let t264 = f64x8::splat(1.0) / t263;
            let t265 = t62 * t264;
            let t270 = -f64x8::splat(1.176575) * t219 - f64x8::splat(0.516475) * t221 - f64x8::splat(0.2103875) * t225 - f64x8::splat(0.104195) * t230;
            let t271 = f64x8::splat(1.0) / t70;
            let t272 = t270 * t271;
            let t278 = t80 * t80;
            let t279 = f64x8::splat(1.0) / t278;
            let t280 = t75 * t279;
            let t285 = -f64x8::splat(0.8630833333333333) * t219 - f64x8::splat(0.301925) * t221 - f64x8::splat(0.05501625) * t225 - f64x8::splat(0.082785) * t230;
            let t286 = f64x8::splat(1.0) / t83;
            let t287 = t285 * t286;
            let t290 = f64x8::splat(0.0005323644333333333) * t4 * t208 * t71 + f64x8::splat(1.0) * t265 * t272 - t211 - t236 + f64x8::splat(0.0001831155503675316) * t4 * t208 * t84 + f64x8::splat(0.5848223397455204) * t280 * t287;
            let t291 = t60 * t290;
            let t292 = t40 * t291;
            let t293 = t257 * t85;
            let t294 = f64x8::splat(0.019751789702565206) * t293;
            let t295 = t60 * t1;
            let t297 = t217 * t207 * t84;
            let t298 = t295 * t297;
            let t299 = f64x8::splat(0.0001831155503675316) * t298;
            let t300 = t60 * t75;
            let t302 = t279 * t285 * t286;
            let t303 = t300 * t302;
            let t304 = f64x8::splat(0.5848223397455204) * t303;
            let t305 = t103 * t172;
            let t306 = f64x8::splat(1.0) / t47;
            let t309 = ((t44).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t306 * t248));
            let t310 = f64x8::splat(1.0) / t52;
            let t313 = ((t51).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t310 * t252));
            let t315 = t309 / f64x8::splat(2.0) + t313 / f64x8::splat(2.0);
            let t317 = t95 * t305 * t315;
            let t318 = f64x8::splat(0.007753464430471029) * t317;
            let t319 = t37 * t7;
            let t321 = f64x8::splat(1.0) / t8 / t319;
            let t322 = t108 * t321;
            let t323 = t322 * t56;
            let t325 = f64x8::splat(7.0) / f64x8::splat(288.0) * t323 * t117;
            let t326 = t56 * t121;
            let t327 = t111 * t326;
            let t328 = t19 * t115;
            let t329 = t5 * t315;
            let t330 = t328 * t329;
            let t333 = t105 * t157;
            let t334 = t134 * t136;
            let t335 = t333 * t334;
            let t336 = t211 + t236 + t240 - t245 + t259 + t292 + t294 - t299 - t304;
            let t340 = t120 * t138;
            let t341 = t124 * t315;
            let t344 = -f64x8::splat(128.97460341341235) * t336 * t121 * t124 + f64x8::splat(386.92381024023706) * t340 * t341;
            let t346 = t6 * t344 * t127;
            let t347 = t161 * t346;
            let t351 = f64x8::splat(1.0) / t22 / t241;
            let t352 = t131 * t351;
            let t353 = t130 * t352;
            let t355 = f64x8::splat(0.012677527172608605) * t353 * t142;
            let t356 = t137 * t102;
            let t357 = f64x8::splat(1.0) / t356;
            let t358 = t136 * t357;
            let t359 = t141 * t315;
            let t360 = t358 * t359;
            let t363 = -t325 - t327 * t330 / f64x8::splat(48.0) - f64x8::splat(0.0027166129655589867) * t335 * t347 - t355 - f64x8::splat(0.010866451862235947) * t135 * t360;
            let t367 = t166 * t166;
            let t368 = f64x8::splat(1.0) / t367;
            let t369 = t145 * t368;
            let t370 = t157 * t108;
            let t372 = t146 * t370 * t110;
            let t373 = t56 * t113;
            let t374 = t373 * t115;
            let t376 = t5 * t344 * t127;
            let t377 = t374 * t376;
            let t380 = t321 * t56;
            let t383 = f64x8::splat(0.2028404347617377) * t148 * t380 * t151;
            let t385 = t146 * t147 * t110;
            let t387 = t326 * t116 * t315;
            let t391 = f64x8::splat(1.0) / t156 / t128;
            let t392 = t391 * t131;
            let t394 = t155 * t392 * t133;
            let t395 = t139 * t140;
            let t396 = t395 * t346;
            let t399 = t351 * t136;
            let t400 = t399 * t162;
            let t402 = f64x8::splat(0.03526649312085494) * t159 * t400;
            let t404 = t155 * t158 * t133;
            let t407 = -f64x8::splat(0.08693161489788757) * t372 * t377 - t383 - f64x8::splat(0.17386322979577515) * t385 * t387 - f64x8::splat(0.015114211337509259) * t394 * t396 - t402 - f64x8::splat(0.030228422675018518) * t404 * t360;
            let t411 = f64x8::splat(2.7818116767324024) * t106 * t363 * t167 - f64x8::splat(2.7818116767324024) * t106 * t369 * t407;
            let t413 = f64x8::splat(1.0) / t171;
            let t415 = t95 * t104 * t411 * t413;
            let t416 = f64x8::splat(0.002584488143490343) * t415;
            let t419 = -f64x8::splat(1.9388333333333334) * t221 - f64x8::splat(0.0012315) * t230;
            let t421 = t182 * t182;
            let t422 = f64x8::splat(1.0) / t421;
            let t423 = t179 * t422;
            let t426 = -f64x8::splat(726.9166666666666) * t221 - f64x8::splat(78.66666666666667) * t230;
            let t428 = t419 * t183 - t423 * t426;
            let t429 = t428 * t102;
            let t430 = t429 * t108;
            let t431 = t176 * t430;
            let t432 = t431 * t203;
            let t433 = t432 / f64x8::splat(2.0);
            let t434 = t185 * t315;
            let t435 = t434 * t108;
            let t436 = t176 * t435;
            let t437 = t436 * t203;
            let t438 = t437 / f64x8::splat(2.0);
            let t439 = t380 * t202;
            let t440 = t188 * t439;
            let t441 = f64x8::splat(7.0) / f64x8::splat(6.0) * t440;
            let t442 = t149 * t115;
            let t444 = f64x8::splat(1.0) / t22 / t319;
            let t448 = f64x8::splat(100.0) / f64x8::splat(27.0) * t193 * t444 * t103 * t197;
            let t449 = t195 * t102;
            let t450 = t197 * t315;
            let t454 = t448 - f64x8::splat(25.0) / f64x8::splat(9.0) * t193 * t449 * t450;
            let t456 = t5 * t454 * t201;
            let t457 = t442 * t456;
            let t458 = t188 * t457;
            let t459 = t458 / f64x8::splat(2.0);
            let t460 = t211 + t236 + t240 - t245 + t259 + t292 + t294 - t299 - t304 + t318 + t416 + t433 + t438 - t441 + t459;
            let tvrho0 = t7 * t460 + t175 + t205 - t33 + t89 + t91;
            acc_vrho_0 = tvrho0;
            let t462 = -t41 - t247;
            let t465 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t462));
            let t466 = -t462;
            let t469 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t466));
            let t471 = (t465 + t469) * t59;
            let t472 = t471 * t87;
            let t473 = t40 * t472;
            let t474 = t471 * t85;
            let t475 = f64x8::splat(0.019751789702565206) * t474;
            let t478 = ((t44).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t306 * t462));
            let t481 = ((t51).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t310 * t466));
            let t483 = t478 / f64x8::splat(2.0) + t481 / f64x8::splat(2.0);
            let t485 = t95 * t305 * t483;
            let t486 = f64x8::splat(0.007753464430471029) * t485;
            let t487 = t5 * t483;
            let t488 = t328 * t487;
            let t491 = t211 + t236 - t240 - t245 + t473 + t292 + t475 - t299 - t304;
            let t495 = t124 * t483;
            let t498 = -f64x8::splat(128.97460341341235) * t491 * t121 * t124 + f64x8::splat(386.92381024023706) * t340 * t495;
            let t500 = t6 * t498 * t127;
            let t501 = t161 * t500;
            let t504 = t141 * t483;
            let t505 = t358 * t504;
            let t508 = -t325 - t327 * t488 / f64x8::splat(48.0) - f64x8::splat(0.0027166129655589867) * t335 * t501 - t355 - f64x8::splat(0.010866451862235947) * t135 * t505;
            let t513 = t5 * t498 * t127;
            let t514 = t374 * t513;
            let t517 = t116 * t483;
            let t518 = t326 * t517;
            let t521 = t395 * t500;
            let t526 = -f64x8::splat(0.08693161489788757) * t372 * t514 - t383 - f64x8::splat(0.17386322979577515) * t385 * t518 - f64x8::splat(0.015114211337509259) * t394 * t521 - t402 - f64x8::splat(0.030228422675018518) * t404 * t505;
            let t530 = f64x8::splat(2.7818116767324024) * t106 * t508 * t167 - f64x8::splat(2.7818116767324024) * t106 * t369 * t526;
            let t533 = t95 * t104 * t530 * t413;
            let t534 = f64x8::splat(0.002584488143490343) * t533;
            let t535 = t185 * t483;
            let t537 = t176 * t535 * t108;
            let t538 = t537 * t203;
            let t539 = t538 / f64x8::splat(2.0);
            let t540 = t197 * t483;
            let t544 = t448 - f64x8::splat(25.0) / f64x8::splat(9.0) * t193 * t449 * t540;
            let t546 = t5 * t544 * t201;
            let t547 = t442 * t546;
            let t548 = t188 * t547;
            let t549 = t548 / f64x8::splat(2.0);
            let t550 = t211 + t236 - t240 - t245 + t473 + t292 + t475 - t299 - t304 + t486 + t534 + t433 + t539 - t441 + t549;
            let tvrho1 = t7 * t550 + t175 + t205 - t33 + t89 + t91;
            acc_vrho_1 = tvrho1;
            let t553 = t328 * t5;
            let t554 = t149 * t113 * t553;
            let t556 = t108 * t133;
            let t557 = t130 * t556;
            let t558 = t557 * t142;
            let t560 = t554 / f64x8::splat(96.0) + f64x8::splat(0.005433225931117973) * t558;
            let t564 = t129 * t110;
            let t565 = t146 * t564;
            let t566 = t373 * t116;
            let t567 = t565 * t566;
            let t569 = t155 * t370;
            let t570 = t569 * t163;
            let t572 = f64x8::splat(0.08693161489788757) * t567 + f64x8::splat(0.015114211337509259) * t570;
            let t576 = f64x8::splat(2.7818116767324024) * t106 * t560 * t167 - f64x8::splat(2.7818116767324024) * t106 * t369 * t572;
            let t580 = f64x8::splat(0.002584488143490343) * t95 * t104 * t576 * t413;
            let t581 = t176 * t186;
            let t582 = t581 * t203;
            let t583 = t582 / f64x8::splat(2.0);
            let t585 = f64x8::splat(1.0) / t92 * t93;
            let t586 = t185 * t104;
            let t587 = t586 * t108;
            let t588 = t585 * t587;
            let t589 = t242 * t136;
            let t591 = t6 * t191 * t201;
            let t592 = t589 * t591;
            let t593 = t588 * t592;
            let t594 = f64x8::splat(25.0) / f64x8::splat(36.0) * t593;
            let tvsigma0 = t7 * (t580 + t583 - t594);
            acc_vsigma_0 = tvsigma0;
            let t598 = t554 / f64x8::splat(48.0) + f64x8::splat(0.010866451862235947) * t558;
            let t604 = f64x8::splat(0.17386322979577515) * t567 + f64x8::splat(0.030228422675018518) * t570;
            let t608 = f64x8::splat(2.7818116767324024) * t106 * t598 * t167 - f64x8::splat(2.7818116767324024) * t106 * t369 * t604;
            let t609 = t104 * t608;
            let t612 = f64x8::splat(0.002584488143490343) * t95 * t609 * t413;
            let t613 = f64x8::splat(25.0) / f64x8::splat(18.0) * t593;
            let tvsigma1 = t7 * (t612 + t582 - t613);
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
