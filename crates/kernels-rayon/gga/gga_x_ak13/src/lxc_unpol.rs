//! GGA_X_AK13 lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ak13.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ak13_lxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
    param_B1: f64,
    param_B2: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_B1 = f64x8::splat(param_B1);
    let param_B2 = f64x8::splat(param_B2);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v3rho2sigma = V_ZERO;
        let mut acc_v3rhosigma2 = V_ZERO;
        let mut acc_v3sigma3 = V_ZERO;
        let mut acc_v4rho4 = V_ZERO;
        let mut acc_v4rho3sigma = V_ZERO;
        let mut acc_v4rho2sigma2 = V_ZERO;
        let mut acc_v4rhosigma3 = V_ZERO;
        let mut acc_v4sigma4 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = f64x8::splat(M_CBRT6);
            let t21 = t20 * t20;
            let t23 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t24 = (simd::cbrt(t23));
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = param_B1 * t21 * t25;
            let t27 = ((v_sigma).sqrt());
            let t28 = f64x8::splat(M_CBRT2);
            let t29 = t27 * t28;
            let t31 = f64x8::splat(1.0) / t18 / v_rho;
            let t32 = t21 * t25;
            let t36 = f64x8::splat(1.0) + t32 * t29 * t31 / f64x8::splat(12.0);
            let t37 = (simd::ln(t36));
            let t38 = t31 * t37;
            let t43 = param_B2 * t21 * t25;
            let t44 = f64x8::splat(1.0) + t37;
            let t45 = (simd::ln(t44));
            let t46 = t31 * t45;
            let t50 = f64x8::splat(1.0) + t26 * t29 * t38 / f64x8::splat(12.0) + t43 * t29 * t46 / f64x8::splat(12.0);
            let t54 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t50));
            let tzk0 = f64x8::splat(2.0) * t54;
            acc_zk = tzk0;
            let t55 = t18 * t18;
            let t57 = t17 / t55;
            let t61 = v_rho * v_rho;
            let t63 = f64x8::splat(1.0) / t18 / t61;
            let t64 = t63 * t37;
            let t69 = t24 * t24;
            let t70 = f64x8::splat(1.0) / t69;
            let t71 = param_B1 * t20 * t70;
            let t72 = t28 * t28;
            let t73 = v_sigma * t72;
            let t74 = t61 * v_rho;
            let t76 = f64x8::splat(1.0) / t55 / t74;
            let t77 = f64x8::splat(1.0) / t36;
            let t78 = t76 * t77;
            let t82 = t63 * t45;
            let t86 = param_B2 * t20;
            let t88 = t86 * t70 * v_sigma;
            let t89 = t72 * t76;
            let t90 = f64x8::splat(1.0) / t44;
            let t91 = t77 * t90;
            let t92 = t89 * t91;
            let t95 = -t26 * t29 * t64 / f64x8::splat(9.0) - t71 * t73 * t78 / f64x8::splat(18.0) - t43 * t29 * t82 / f64x8::splat(9.0) - t88 * t92 / f64x8::splat(18.0);
            let t100 = ((t2).select(f64x8::splat(0.0), -t6 * t57 * t50 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t95));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t100 + f64x8::splat(2.0) * t54;
            acc_vrho = tvrho0;
            let t103 = f64x8::splat(1.0) / t27;
            let t104 = t103 * t28;
            let t109 = f64x8::splat(1.0) / t55 / t61;
            let t110 = t72 * t109;
            let t117 = t86 * t70;
            let t118 = t110 * t91;
            let t121 = t26 * t104 * t38 / f64x8::splat(24.0) + t71 * t110 * t77 / f64x8::splat(48.0) + t43 * t104 * t46 / f64x8::splat(24.0) + t117 * t118 / f64x8::splat(48.0);
            let t125 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t121));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t125;
            acc_vsigma = tvsigma0;
            let t130 = t17 / t55 / v_rho;
            let t138 = f64x8::splat(1.0) / t18 / t74;
            let t139 = t138 * t37;
            let t143 = t61 * t61;
            let t145 = f64x8::splat(1.0) / t55 / t143;
            let t146 = t145 * t77;
            let t150 = f64x8::splat(1.0) / t23;
            let t151 = param_B1 * t150;
            let t152 = t27 * v_sigma;
            let t153 = t143 * t61;
            let t154 = f64x8::splat(1.0) / t153;
            let t156 = t36 * t36;
            let t157 = f64x8::splat(1.0) / t156;
            let t161 = t138 * t45;
            let t165 = t72 * t145;
            let t166 = t165 * t91;
            let t169 = param_B2 * t150;
            let t170 = t169 * t152;
            let t171 = t154 * t157;
            let t172 = t171 * t90;
            let t175 = t44 * t44;
            let t176 = f64x8::splat(1.0) / t175;
            let t177 = t171 * t176;
            let t180 = f64x8::splat(7.0) / f64x8::splat(27.0) * t26 * t29 * t139 + f64x8::splat(5.0) / f64x8::splat(18.0) * t71 * t73 * t146 - f64x8::splat(2.0) / f64x8::splat(27.0) * t151 * t152 * t154 * t157 + f64x8::splat(7.0) / f64x8::splat(27.0) * t43 * t29 * t161 + f64x8::splat(5.0) / f64x8::splat(18.0) * t88 * t166 - f64x8::splat(2.0) / f64x8::splat(27.0) * t170 * t172 - f64x8::splat(2.0) / f64x8::splat(27.0) * t170 * t177;
            let t185 = ((t2).select(f64x8::splat(0.0), t6 * t130 * t50 / f64x8::splat(12.0) - t6 * t57 * t95 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t180));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t185 + f64x8::splat(4.0) * t100;
            acc_v2rho2 = tv2rho20;
            let t197 = t143 * v_rho;
            let t198 = f64x8::splat(1.0) / t197;
            let t199 = t198 * t157;
            let t208 = t169 * t198;
            let t209 = t157 * t90;
            let t210 = t209 * t27;
            let t213 = t157 * t176;
            let t214 = t213 * t27;
            let t217 = -t26 * t104 * t64 / f64x8::splat(18.0) - t71 * t89 * t77 / f64x8::splat(12.0) + t151 * t199 * t27 / f64x8::splat(36.0) - t43 * t104 * t82 / f64x8::splat(18.0) - t117 * t92 / f64x8::splat(12.0) + t208 * t210 / f64x8::splat(36.0) + t208 * t214 / f64x8::splat(36.0);
            let t222 = ((t2).select(f64x8::splat(0.0), -t6 * t57 * t121 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t217));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t222 + f64x8::splat(2.0) * t125;
            acc_v2rhosigma = tv2rhosigma0;
            let t225 = f64x8::splat(1.0) / t152;
            let t226 = t225 * t28;
            let t230 = f64x8::splat(1.0) / v_sigma;
            let t231 = t230 * t72;
            let t232 = t109 * t77;
            let t236 = f64x8::splat(1.0) / t143;
            let t237 = t236 * t157;
            let t245 = t86 * t70 * t230;
            let t248 = t169 * t236;
            let t255 = -t26 * t226 * t38 / f64x8::splat(48.0) + t71 * t231 * t232 / f64x8::splat(96.0) - t151 * t237 * t103 / f64x8::splat(96.0) - t43 * t226 * t46 / f64x8::splat(48.0) + t245 * t118 / f64x8::splat(96.0) - t248 * t209 * t103 / f64x8::splat(96.0) - t248 * t213 * t103 / f64x8::splat(96.0);
            let t259 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t255));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t259;
            acc_v2sigma2 = tv2sigma20;
            let t262 = t17 * t109;
            let t273 = f64x8::splat(1.0) / t18 / t143;
            let t274 = t273 * t37;
            let t279 = f64x8::splat(1.0) / t55 / t197;
            let t284 = t143 * t74;
            let t285 = f64x8::splat(1.0) / t284;
            let t290 = v_sigma * v_sigma;
            let t291 = t143 * t143;
            let t293 = f64x8::splat(1.0) / t18 / t291;
            let t294 = t290 * t293;
            let t297 = f64x8::splat(1.0) / t156 / t36;
            let t299 = t25 * t28;
            let t300 = t297 * t21 * t299;
            let t303 = t273 * t45;
            let t307 = t72 * t279;
            let t308 = t307 * t91;
            let t311 = t285 * t157;
            let t318 = t169 * t294;
            let t320 = t32 * t28;
            let t321 = t297 * t90 * t320;
            let t325 = t297 * t176 * t320;
            let t329 = f64x8::splat(1.0) / t175 / t44;
            let t331 = t297 * t329 * t320;
            let t334 = -f64x8::splat(70.0) / f64x8::splat(81.0) * t26 * t29 * t274 - f64x8::splat(119.0) / f64x8::splat(81.0) * t71 * t73 * t279 * t77 + f64x8::splat(22.0) / f64x8::splat(27.0) * t151 * t152 * t285 * t157 - f64x8::splat(4.0) / f64x8::splat(243.0) * t151 * t294 * t300 - f64x8::splat(70.0) / f64x8::splat(81.0) * t43 * t29 * t303 - f64x8::splat(119.0) / f64x8::splat(81.0) * t88 * t308 + f64x8::splat(22.0) / f64x8::splat(27.0) * t170 * t311 * t90 + f64x8::splat(22.0) / f64x8::splat(27.0) * t170 * t311 * t176 - f64x8::splat(4.0) / f64x8::splat(243.0) * t318 * t321 - f64x8::splat(2.0) / f64x8::splat(81.0) * t318 * t325 - f64x8::splat(4.0) / f64x8::splat(243.0) * t318 * t331;
            let t339 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t262 * t50 + t6 * t130 * t95 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t57 * t180 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t334));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t339 + f64x8::splat(6.0) * t185;
            acc_v3rho3 = tv3rho30;
            let t359 = f64x8::splat(1.0) / t18 / t284;
            let t360 = t359 * t297;
            let t363 = v_sigma * t21 * t299;
            let t371 = t169 * t154;
            let t376 = t169 * t360;
            let t378 = t90 * v_sigma * t320;
            let t382 = t176 * v_sigma * t320;
            let t386 = t329 * v_sigma * t320;
            let t389 = f64x8::splat(7.0) / f64x8::splat(54.0) * t26 * t104 * t139 + f64x8::splat(37.0) / f64x8::splat(108.0) * t71 * t165 * t77 - t151 * t171 * t27 / f64x8::splat(4.0) + t151 * t360 * t363 / f64x8::splat(162.0) + f64x8::splat(7.0) / f64x8::splat(54.0) * t43 * t104 * t161 + f64x8::splat(37.0) / f64x8::splat(108.0) * t117 * t166 - t371 * t210 / f64x8::splat(4.0) - t371 * t214 / f64x8::splat(4.0) + t376 * t378 / f64x8::splat(162.0) + t376 * t382 / f64x8::splat(108.0) + t376 * t386 / f64x8::splat(162.0);
            let t394 = ((t2).select(f64x8::splat(0.0), t6 * t130 * t121 / f64x8::splat(12.0) - t6 * t57 * t217 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t389));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t394 + f64x8::splat(4.0) * t222;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t411 = f64x8::splat(1.0) / t18 / t153;
            let t420 = t169 * t103;
            let t421 = t199 * t90;
            let t424 = t199 * t176;
            let t427 = t411 * t297;
            let t428 = t169 * t427;
            let t430 = t90 * t21 * t299;
            let t434 = t176 * t21 * t299;
            let t438 = t329 * t21 * t299;
            let t441 = t26 * t226 * t64 / f64x8::splat(36.0) - t71 * t231 * t78 / f64x8::splat(72.0) + t151 * t103 * t198 * t157 / f64x8::splat(18.0) - t151 * t411 * t300 / f64x8::splat(432.0) + t43 * t226 * t82 / f64x8::splat(36.0) - t245 * t92 / f64x8::splat(72.0) + t420 * t421 / f64x8::splat(18.0) + t420 * t424 / f64x8::splat(18.0) - t428 * t430 / f64x8::splat(432.0) - t428 * t434 / f64x8::splat(288.0) - t428 * t438 / f64x8::splat(432.0);
            let t446 = ((t2).select(f64x8::splat(0.0), -t6 * t57 * t255 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t441));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t446 + f64x8::splat(2.0) * t259;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t449 = t27 * t290;
            let t450 = f64x8::splat(1.0) / t449;
            let t451 = t450 * t28;
            let t455 = f64x8::splat(1.0) / t290;
            let t456 = t455 * t72;
            let t461 = f64x8::splat(1.0) / t18 / t197;
            let t462 = t461 * t297;
            let t463 = t151 * t462;
            let t465 = t230 * t21 * t299;
            let t472 = t86 * t70 * t455;
            let t475 = t169 * t462;
            let t477 = t90 * t230 * t320;
            let t481 = t176 * t230 * t320;
            let t485 = t329 * t230 * t320;
            let t488 = t26 * t451 * t38 / f64x8::splat(32.0) - t71 * t456 * t232 / f64x8::splat(64.0) + t463 * t465 / f64x8::splat(1152.0) + t43 * t451 * t46 / f64x8::splat(32.0) - t472 * t118 / f64x8::splat(64.0) + t475 * t477 / f64x8::splat(1152.0) + t475 * t481 / f64x8::splat(768.0) + t475 * t485 / f64x8::splat(1152.0);
            let t492 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t488));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t492;
            acc_v3sigma3 = tv3sigma30;
            let t508 = f64x8::splat(1.0) / t291;
            let t513 = t508 * t157;
            let t520 = t291 * v_rho;
            let t523 = t290 / t18 / t520;
            let t530 = t449 / t55 / t291 / t61;
            let t532 = t156 * t156;
            let t533 = f64x8::splat(1.0) / t532;
            let t535 = t70 * t72;
            let t544 = f64x8::splat(1.0) / t55 / t153;
            let t557 = t169 * t523;
            let t564 = t169 * t530;
            let t566 = t20 * t70;
            let t567 = t566 * t72;
            let t579 = t175 * t175;
            let t580 = f64x8::splat(1.0) / t579;
            let t585 = -f64x8::splat(1862.0) / f64x8::splat(243.0) * t151 * t152 * t508 * t157 - f64x8::splat(1862.0) / f64x8::splat(243.0) * t170 * t513 * t90 - f64x8::splat(1862.0) / f64x8::splat(243.0) * t170 * t513 * t176 + f64x8::splat(232.0) / f64x8::splat(729.0) * t151 * t523 * t300 - f64x8::splat(8.0) / f64x8::splat(243.0) * t151 * t530 * t533 * t20 * t535 + f64x8::splat(910.0) / f64x8::splat(243.0) * t43 * t29 * t461 * t45 + f64x8::splat(721.0) / f64x8::splat(81.0) * t88 * t72 * t544 * t91 + f64x8::splat(910.0) / f64x8::splat(243.0) * t26 * t29 * t461 * t37 + f64x8::splat(721.0) / f64x8::splat(81.0) * t71 * t73 * t544 * t77 + f64x8::splat(232.0) / f64x8::splat(729.0) * t557 * t321 + f64x8::splat(116.0) / f64x8::splat(243.0) * t557 * t325 + f64x8::splat(232.0) / f64x8::splat(729.0) * t557 * t331 - f64x8::splat(8.0) / f64x8::splat(243.0) * t564 * t533 * t90 * t567 - f64x8::splat(44.0) / f64x8::splat(729.0) * t564 * t533 * t176 * t567 - f64x8::splat(16.0) / f64x8::splat(243.0) * t564 * t533 * t329 * t567 - f64x8::splat(8.0) / f64x8::splat(243.0) * t564 * t533 * t580 * t567;
            let t590 = ((t2).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(27.0) * t6 * t17 * t76 * t50 - f64x8::splat(5.0) / f64x8::splat(9.0) * t6 * t262 * t95 + t6 * t130 * t180 / f64x8::splat(2.0) - t6 * t57 * t334 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t585));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t590 + f64x8::splat(8.0) * t339;
            acc_v4rho4 = tv4rho40;
            let t606 = t293 * t297;
            let t612 = f64x8::splat(1.0) / t55 / t520 * t533;
            let t627 = t169 * t285;
            let t634 = t169 * t606;
            let t641 = t169 * t612;
            let t658 = -f64x8::splat(35.0) / f64x8::splat(81.0) * t26 * t104 * t274 - f64x8::splat(49.0) / f64x8::splat(486.0) * t151 * t606 * t363 + t151 * t612 * t152 * t20 * t535 / f64x8::splat(81.0) - f64x8::splat(35.0) / f64x8::splat(81.0) * t43 * t104 * t303 + f64x8::splat(317.0) / f64x8::splat(162.0) * t151 * t311 * t27 - f64x8::splat(91.0) / f64x8::splat(54.0) * t71 * t307 * t77 + f64x8::splat(317.0) / f64x8::splat(162.0) * t627 * t210 + f64x8::splat(317.0) / f64x8::splat(162.0) * t627 * t214 - f64x8::splat(91.0) / f64x8::splat(54.0) * t117 * t308 - f64x8::splat(49.0) / f64x8::splat(486.0) * t634 * t378 - f64x8::splat(49.0) / f64x8::splat(324.0) * t634 * t382 - f64x8::splat(49.0) / f64x8::splat(486.0) * t634 * t386 + t641 * t90 * t152 * t567 / f64x8::splat(81.0) + f64x8::splat(11.0) / f64x8::splat(486.0) * t641 * t176 * t152 * t567 + f64x8::splat(2.0) / f64x8::splat(81.0) * t641 * t329 * t152 * t567 + t641 * t580 * t152 * t567 / f64x8::splat(81.0);
            let t663 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t262 * t121 + t6 * t130 * t217 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t57 * t389 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t658));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t663 + f64x8::splat(6.0) * t394;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t675 = f64x8::splat(1.0) / t55 / t291 * t533;
            let t676 = t169 * t675;
            let t678 = t535 * t27;
            let t727 = -t676 * t329 * t20 * t678 / f64x8::splat(108.0) - t676 * t580 * t20 * t678 / f64x8::splat(216.0) - t151 * t675 * t566 * t72 * t27 / f64x8::splat(216.0) - f64x8::splat(7.0) / f64x8::splat(108.0) * t43 * t226 * t161 + f64x8::splat(7.0) / f64x8::splat(216.0) * t245 * t166 + f64x8::splat(35.0) / f64x8::splat(1296.0) * t376 * t430 + f64x8::splat(35.0) / f64x8::splat(864.0) * t376 * t434 + f64x8::splat(35.0) / f64x8::splat(1296.0) * t376 * t438 - f64x8::splat(7.0) / f64x8::splat(108.0) * t26 * t226 * t139 - f64x8::splat(8.0) / f64x8::splat(27.0) * t151 * t103 * t154 * t157 - f64x8::splat(8.0) / f64x8::splat(27.0) * t420 * t172 - f64x8::splat(8.0) / f64x8::splat(27.0) * t420 * t177 + f64x8::splat(7.0) / f64x8::splat(216.0) * t71 * t231 * t146 + f64x8::splat(35.0) / f64x8::splat(1296.0) * t151 * t359 * t300 - t676 * t90 * t20 * t678 / f64x8::splat(216.0) - f64x8::splat(11.0) / f64x8::splat(1296.0) * t676 * t176 * t20 * t678;
            let t732 = ((t2).select(f64x8::splat(0.0), t6 * t130 * t255 / f64x8::splat(12.0) - t6 * t57 * t441 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t727));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t732 + f64x8::splat(4.0) * t446;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t753 = f64x8::splat(1.0) / t55 / t284 * t533;
            let t764 = t169 * t225;
            let t771 = t169 * t753;
            let t792 = -t26 * t451 * t64 / f64x8::splat(24.0) + t71 * t456 * t78 / f64x8::splat(48.0) - t151 * t225 * t198 * t157 / f64x8::splat(48.0) - t151 * t427 * t465 / f64x8::splat(216.0) + t151 * t753 * t103 * t20 * t535 / f64x8::splat(576.0) - t43 * t451 * t82 / f64x8::splat(24.0) + t472 * t92 / f64x8::splat(48.0) - t764 * t421 / f64x8::splat(48.0) - t764 * t424 / f64x8::splat(48.0) - t428 * t477 / f64x8::splat(216.0) + t771 * t90 * t103 * t567 / f64x8::splat(576.0) + f64x8::splat(11.0) / f64x8::splat(3456.0) * t771 * t176 * t103 * t567 - t428 * t481 / f64x8::splat(144.0) + t771 * t329 * t103 * t567 / f64x8::splat(288.0) - t428 * t485 / f64x8::splat(216.0) + t771 * t580 * t103 * t567 / f64x8::splat(576.0);
            let t797 = ((t2).select(f64x8::splat(0.0), -t6 * t57 * t488 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t792));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t797 + f64x8::splat(2.0) * t492;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t800 = t290 * v_sigma;
            let t803 = f64x8::splat(1.0) / t27 / t800 * t28;
            let t807 = f64x8::splat(1.0) / t800;
            let t816 = t544 * t533;
            let t833 = t169 * t450;
            let t840 = t169 * t816;
            let t869 = -f64x8::splat(5.0) / f64x8::splat(64.0) * t26 * t803 * t38 + f64x8::splat(5.0) / f64x8::splat(128.0) * t71 * t807 * t72 * t232 + t151 * t450 * t236 * t157 / f64x8::splat(128.0) - t151 * t816 * t225 * t20 * t535 / f64x8::splat(1536.0) - t463 * t455 * t21 * t299 / f64x8::splat(1152.0) - f64x8::splat(5.0) / f64x8::splat(64.0) * t43 * t803 * t46 + f64x8::splat(5.0) / f64x8::splat(128.0) * t86 * t70 * t807 * t118 + t833 * t237 * t90 / f64x8::splat(128.0) + t833 * t237 * t176 / f64x8::splat(128.0) - t840 * t90 * t225 * t567 / f64x8::splat(1536.0) - f64x8::splat(11.0) / f64x8::splat(9216.0) * t840 * t176 * t225 * t567 - t475 * t90 * t455 * t320 / f64x8::splat(1152.0) - t840 * t329 * t225 * t567 / f64x8::splat(768.0) - t475 * t176 * t455 * t320 / f64x8::splat(768.0) - t840 * t580 * t225 * t567 / f64x8::splat(1536.0) - t475 * t329 * t455 * t320 / f64x8::splat(1152.0);
            let t873 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t869));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t873;
            acc_v4sigma4 = tv4sigma40;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho3.into(); v3rho3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho2sigma.into(); v3rho2sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rhosigma2.into(); v3rhosigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3sigma3.into(); v3sigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho4.into(); v4rho4[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho3sigma.into(); v4rho3sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho2sigma2.into(); v4rho2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rhosigma3.into(); v4rhosigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4sigma4.into(); v4sigma4[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
