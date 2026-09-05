//! GGA_C_PBE vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pbe.c`
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
pub fn gga_c_pbe_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_gamma: f64,
    param_BB: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_gamma = f64x8::splat(param_gamma);
    let param_BB = f64x8::splat(param_BB);
    let param_beta = f64x8::splat(param_beta);
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
            let t30 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t27;
            let t31 = (simd::ln(t30));
            let t33 = f64x8::splat(0.0621814) * t13 * t31;
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
            let t70 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t67;
            let t71 = (simd::ln(t70));
            let t75 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t80 = f64x8::splat(5.1785) * t14 + f64x8::splat(0.905775) * t11 + f64x8::splat(0.1100325) * t17 + f64x8::splat(0.1241775) * t25;
            let t83 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t80;
            let t84 = (simd::ln(t83));
            let t85 = t75 * t84;
            let t87 = -f64x8::splat(0.0310907) * t62 * t71 + t33 - f64x8::splat(0.0197516734986138) * t85;
            let t88 = t60 * t87;
            let t89 = t40 * t88;
            let t91 = f64x8::splat(0.0197516734986138) * t60 * t85;
            let t92 = t45 * t45;
            let t93 = t47 * t47;
            let t94 = ((t44).select(t92, t93));
            let t95 = t52 * t52;
            let t96 = ((t51).select(t92, t95));
            let t98 = t94 / f64x8::splat(2.0) + t96 / f64x8::splat(2.0);
            let t99 = t98 * t98;
            let t100 = t99 * t98;
            let t101 = param_gamma * t100;
            let t103 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t105 = f64x8::splat(1.0) / t8 / t37;
            let t106 = t103 * t105;
            let t108 = f64x8::splat(1.0) / t99;
            let t110 = f64x8::splat(1.0) / t3;
            let t111 = t110 * t5;
            let t112 = t108 * t19 * t111;
            let t115 = param_BB * param_beta;
            let t116 = f64x8::splat(1.0) / param_gamma;
            let t118 = (-t33 + t89 + t91) * t116;
            let t119 = f64x8::splat(1.0) / t100;
            let t121 = (simd::exp(-t118 * t119));
            let t122 = t121 - f64x8::splat(1.0);
            let t123 = f64x8::splat(1.0) / t122;
            let t124 = t116 * t123;
            let t125 = t103 * t103;
            let t127 = t115 * t124 * t125;
            let t129 = f64x8::splat(1.0) / t22 / t38;
            let t130 = t56 * t56;
            let t131 = t129 * t130;
            let t132 = t99 * t99;
            let t133 = f64x8::splat(1.0) / t132;
            let t134 = t131 * t133;
            let t135 = f64x8::splat(1.0) / t20;
            let t136 = t1 * t135;
            let t137 = t136 * t6;
            let t138 = t134 * t137;
            let t141 = t106 * t56 * t112 / f64x8::splat(96.0) + t127 * t138 / f64x8::splat(3072.0);
            let t142 = param_beta * t141;
            let t143 = param_beta * t116;
            let t146 = t143 * t123 * t141 + f64x8::splat(1.0);
            let t147 = f64x8::splat(1.0) / t146;
            let t148 = t116 * t147;
            let t150 = t142 * t148 + f64x8::splat(1.0);
            let t151 = (simd::ln(t150));
            let t152 = t101 * t151;
            let tzk0 = -t33 + t89 + t91 + t152;
            acc_zk = tzk0;
            let t154 = f64x8::splat(1.0) / t8 / t7;
            let t155 = t6 * t154;
            let t157 = t4 * t155 * t31;
            let t158 = f64x8::splat(0.0011073470983333333) * t157;
            let t159 = t27 * t27;
            let t160 = f64x8::splat(1.0) / t159;
            let t161 = t13 * t160;
            let t163 = f64x8::splat(1.0) / t14 * t1;
            let t164 = t3 * t6;
            let t165 = t164 * t154;
            let t166 = t163 * t165;
            let t168 = t4 * t155;
            let t170 = ((t11).sqrt());
            let t171 = t170 * t1;
            let t172 = t171 * t165;
            let t177 = t21 * t5 / t22 / t7;
            let t179 = -f64x8::splat(0.632975) * t166 - f64x8::splat(0.29896666666666666) * t168 - f64x8::splat(0.1023875) * t172 - f64x8::splat(0.08215666666666667) * t177;
            let t180 = f64x8::splat(1.0) / t30;
            let t181 = t179 * t180;
            let t182 = t161 * t181;
            let t183 = f64x8::splat(1.0) * t182;
            let t184 = t35 * t34;
            let t185 = t184 * t39;
            let t186 = t185 * t88;
            let t187 = f64x8::splat(4.0) * t186;
            let t188 = t38 * t7;
            let t189 = f64x8::splat(1.0) / t188;
            let t190 = t36 * t189;
            let t191 = t190 * t88;
            let t192 = f64x8::splat(4.0) * t191;
            let t193 = f64x8::splat(1.0) / t37;
            let t194 = t34 * t193;
            let t195 = t41 - t194;
            let t198 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t195));
            let t199 = -t195;
            let t202 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t199));
            let t204 = (t198 + t202) * t59;
            let t205 = t204 * t87;
            let t206 = t40 * t205;
            let t210 = t67 * t67;
            let t211 = f64x8::splat(1.0) / t210;
            let t212 = t62 * t211;
            let t217 = -f64x8::splat(1.176575) * t166 - f64x8::splat(0.516475) * t168 - f64x8::splat(0.2103875) * t172 - f64x8::splat(0.104195) * t177;
            let t218 = f64x8::splat(1.0) / t70;
            let t219 = t217 * t218;
            let t225 = t80 * t80;
            let t226 = f64x8::splat(1.0) / t225;
            let t227 = t75 * t226;
            let t232 = -f64x8::splat(0.8630833333333333) * t166 - f64x8::splat(0.301925) * t168 - f64x8::splat(0.05501625) * t172 - f64x8::splat(0.082785) * t177;
            let t233 = f64x8::splat(1.0) / t83;
            let t234 = t232 * t233;
            let t237 = f64x8::splat(0.0005323764196666666) * t4 * t155 * t71 + f64x8::splat(1.0) * t212 * t219 - t158 - t183 + f64x8::splat(0.00018311447306006544) * t4 * t155 * t84 + f64x8::splat(0.5848223622634646) * t227 * t234;
            let t238 = t60 * t237;
            let t239 = t40 * t238;
            let t240 = t204 * t85;
            let t241 = f64x8::splat(0.0197516734986138) * t240;
            let t242 = t60 * t1;
            let t244 = t164 * t154 * t84;
            let t245 = t242 * t244;
            let t246 = f64x8::splat(0.00018311447306006544) * t245;
            let t247 = t60 * t75;
            let t249 = t226 * t232 * t233;
            let t250 = t247 * t249;
            let t251 = f64x8::splat(0.5848223622634646) * t250;
            let t252 = param_gamma * t99;
            let t253 = f64x8::splat(1.0) / t47;
            let t256 = ((t44).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t253 * t195));
            let t257 = f64x8::splat(1.0) / t52;
            let t260 = ((t51).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t257 * t199));
            let t262 = t256 / f64x8::splat(2.0) + t260 / f64x8::splat(2.0);
            let t263 = t151 * t262;
            let t264 = t252 * t263;
            let t265 = f64x8::splat(3.0) * t264;
            let t266 = t37 * t7;
            let t268 = f64x8::splat(1.0) / t8 / t266;
            let t269 = t103 * t268;
            let t272 = f64x8::splat(7.0) / f64x8::splat(288.0) * t269 * t56 * t112;
            let t273 = t56 * t119;
            let t274 = t106 * t273;
            let t275 = t19 * t110;
            let t276 = t5 * t262;
            let t277 = t275 * t276;
            let t280 = t115 * t116;
            let t281 = t122 * t122;
            let t282 = f64x8::splat(1.0) / t281;
            let t283 = t282 * t125;
            let t285 = t280 * t283 * t129;
            let t286 = t130 * t133;
            let t287 = t286 * t1;
            let t288 = t135 * t6;
            let t290 = (t158 + t183 + t187 - t192 + t206 + t239 + t241 - t246 - t251) * t116;
            let t292 = t133 * t262;
            let t295 = f64x8::splat(3.0) * t118 * t292 - t290 * t119;
            let t296 = t295 * t121;
            let t297 = t288 * t296;
            let t298 = t287 * t297;
            let t302 = f64x8::splat(1.0) / t22 / t188;
            let t303 = t302 * t130;
            let t304 = t303 * t133;
            let t305 = t304 * t137;
            let t307 = f64x8::splat(7.0) / f64x8::splat(4608.0) * t127 * t305;
            let t308 = t123 * t125;
            let t310 = t280 * t308 * t129;
            let t312 = f64x8::splat(1.0) / t132 / t98;
            let t313 = t130 * t312;
            let t314 = t313 * t1;
            let t316 = t314 * t288 * t262;
            let t319 = -t272 - t274 * t277 / f64x8::splat(48.0) - t285 * t298 / f64x8::splat(3072.0) - t307 - t310 * t316 / f64x8::splat(768.0);
            let t320 = param_beta * t319;
            let t322 = t146 * t146;
            let t323 = f64x8::splat(1.0) / t322;
            let t324 = t116 * t323;
            let t325 = t143 * t282;
            let t326 = t141 * t295;
            let t331 = -t325 * t326 * t121 + t143 * t123 * t319;
            let t332 = t324 * t331;
            let t334 = -t142 * t332 + t320 * t148;
            let t335 = f64x8::splat(1.0) / t150;
            let t336 = t334 * t335;
            let t337 = t101 * t336;
            let t338 = t158 + t183 + t187 - t192 + t206 + t239 + t241 - t246 - t251 + t265 + t337;
            let tvrho0 = t7 * t338 + t152 - t33 + t89 + t91;
            acc_vrho_0 = tvrho0;
            let t340 = -t41 - t194;
            let t343 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t340));
            let t344 = -t340;
            let t347 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t344));
            let t349 = (t343 + t347) * t59;
            let t350 = t349 * t87;
            let t351 = t40 * t350;
            let t352 = t349 * t85;
            let t353 = f64x8::splat(0.0197516734986138) * t352;
            let t356 = ((t44).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t253 * t340));
            let t359 = ((t51).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t257 * t344));
            let t361 = t356 / f64x8::splat(2.0) + t359 / f64x8::splat(2.0);
            let t362 = t151 * t361;
            let t363 = t252 * t362;
            let t364 = f64x8::splat(3.0) * t363;
            let t365 = t5 * t361;
            let t366 = t275 * t365;
            let t370 = (t158 + t183 - t187 - t192 + t351 + t239 + t353 - t246 - t251) * t116;
            let t372 = t133 * t361;
            let t375 = f64x8::splat(3.0) * t118 * t372 - t370 * t119;
            let t376 = t375 * t121;
            let t377 = t288 * t376;
            let t378 = t287 * t377;
            let t382 = t314 * t288 * t361;
            let t385 = -t272 - t274 * t366 / f64x8::splat(48.0) - t285 * t378 / f64x8::splat(3072.0) - t307 - t310 * t382 / f64x8::splat(768.0);
            let t386 = param_beta * t385;
            let t388 = t141 * t375;
            let t393 = -t325 * t388 * t121 + t143 * t123 * t385;
            let t394 = t324 * t393;
            let t396 = -t142 * t394 + t386 * t148;
            let t397 = t396 * t335;
            let t398 = t101 * t397;
            let t399 = t158 + t183 - t187 - t192 + t351 + t239 + t353 - t246 - t251 + t364 + t398;
            let tvrho1 = t7 * t399 + t152 - t33 + t89 + t91;
            acc_vrho_1 = tvrho1;
            let t401 = t7 * param_gamma;
            let t402 = t105 * t56;
            let t404 = t275 * t5;
            let t405 = t402 * t108 * t404;
            let t408 = t115 * t124 * t103;
            let t409 = t408 * t138;
            let t411 = t405 / f64x8::splat(96.0) + t409 / f64x8::splat(1536.0);
            let t412 = param_beta * t411;
            let t414 = param_beta * param_beta;
            let t415 = t414 * t141;
            let t416 = param_gamma * param_gamma;
            let t417 = f64x8::splat(1.0) / t416;
            let t418 = t415 * t417;
            let t419 = t323 * t123;
            let t420 = t419 * t411;
            let t422 = t412 * t148 - t418 * t420;
            let tvsigma0 = t401 * t100 * t422 * t335;
            acc_vsigma_0 = tvsigma0;
            let t427 = t405 / f64x8::splat(48.0) + t409 / f64x8::splat(768.0);
            let t428 = param_beta * t427;
            let t430 = t419 * t427;
            let t432 = t428 * t148 - t418 * t430;
            let t433 = t100 * t432;
            let tvsigma1 = t401 * t433 * t335;
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
