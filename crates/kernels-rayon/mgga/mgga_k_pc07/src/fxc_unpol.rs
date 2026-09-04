//! MGGA_K_PC07 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_pc07.c`
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
pub fn mgga_k_pc07_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
    let param_b = f64x8::splat(param_b);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2rholapl = V_ZERO;
        let mut acc_v2rhotau = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v2sigmalapl = V_ZERO;
        let mut acc_v2sigmatau = V_ZERO;
        let mut acc_v2lapl2 = V_ZERO;
        let mut acc_v2lapltau = V_ZERO;
        let mut acc_v2tau2 = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = t4 * t4;
            let t6 = f64x8::splat(M_CBRTPI);
            let t8 = t5 * t6 * f64x8::splat(M_PI);
            let t9 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t10 = zeta_threshold - f64x8::splat(1.0);
            let t12 = ((t9).select(t10, (t9).select(-t10, f64x8::splat(0.0))));
            let t13 = f64x8::splat(1.0) + t12;
            let t15 = (simd::cbrt(zeta_threshold));
            let t16 = t15 * t15;
            let t18 = (simd::cbrt(t13));
            let t19 = t18 * t18;
            let t21 = (((t13).simd_le(zeta_threshold)).select(t16 * zeta_threshold, t19 * t13));
            let t22 = (simd::cbrt(v_rho));
            let t23 = t22 * t22;
            let t24 = t21 * t23;
            let t25 = f64x8::splat(M_CBRT6);
            let t26 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t27 = (simd::cbrt(t26));
            let t28 = t27 * t27;
            let t29 = f64x8::splat(1.0) / t28;
            let t30 = t25 * t29;
            let t31 = f64x8::splat(M_CBRT2);
            let t32 = t31 * t31;
            let t33 = v_sigma * t32;
            let t34 = v_rho * v_rho;
            let t36 = f64x8::splat(1.0) / t23 / t34;
            let t38 = t30 * t33 * t36;
            let t39 = f64x8::splat(5.0) / f64x8::splat(72.0) * t38;
            let t41 = v_lapl * t32;
            let t43 = f64x8::splat(1.0) / t23 / v_rho;
            let t47 = t25 * t25;
            let t49 = f64x8::splat(1.0) / t27 / t26;
            let t50 = t47 * t49;
            let t51 = v_lapl * v_lapl;
            let t52 = t51 * t31;
            let t53 = t34 * v_rho;
            let t55 = f64x8::splat(1.0) / t22 / t53;
            let t58 = t50 * t52 * t55 / f64x8::splat(2916.0);
            let t59 = t50 * v_sigma;
            let t60 = t34 * t34;
            let t62 = f64x8::splat(1.0) / t22 / t60;
            let t63 = t31 * t62;
            let t64 = t63 * v_lapl;
            let t66 = t59 * t64 / f64x8::splat(2592.0);
            let t67 = v_sigma * v_sigma;
            let t68 = t67 * t31;
            let t69 = t60 * v_rho;
            let t71 = f64x8::splat(1.0) / t22 / t69;
            let t74 = t50 * t68 * t71 / f64x8::splat(8748.0);
            let t75 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(648.0) * t38 + f64x8::splat(5.0) / f64x8::splat(54.0) * t30 * t41 * t43 + t58 - t66 + t74;
            let t76 = t58 - t66 + t74;
            let t77 = t76 * t76;
            let t78 = f64x8::splat(1.0) + t39;
            let t79 = t78 * t78;
            let t80 = f64x8::splat(1.0) / t79;
            let t82 = t77 * t80 + f64x8::splat(1.0);
            let t83 = ((t82).sqrt());
            let t84 = f64x8::splat(1.0) / t83;
            let t86 = t75 * t84 - t39;
            let t87 = param_a / f64x8::splat(40.0);
            let t88 = (t86).simd_le(t87);
            let t89 = f64x8::splat(39.0) / f64x8::splat(40.0) * param_a;
            let t90 = (t89).simd_le(t86);
            let t91 = param_a * param_b;
            let t92 = (t86).simd_lt(t87);
            let t93 = ((t92).select(t87, t86));
            let t94 = (t93).simd_lt(t89);
            let t95 = ((t94).select(t93, t89));
            let t96 = f64x8::splat(1.0) / t95;
            let t98 = (simd::exp(-t91 * t96));
            let t99 = param_a - t95;
            let t102 = (simd::exp(-param_a / t99));
            let t103 = f64x8::splat(1.0) + t102;
            let t104 = (simd::pow(t103, param_b));
            let t105 = t98 * t104;
            let t107 = (simd::exp(-param_a * t96));
            let t108 = t107 + t102;
            let t109 = (simd::pow(t108, param_b));
            let t110 = f64x8::splat(1.0) / t109;
            let t111 = t105 * t110;
            let t112 = ((t88).select(f64x8::splat(0.0), (t90).select(f64x8::splat(1.0), t111)));
            let t114 = t86 * t112 + t39;
            let t118 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t114));
            let tzk0 = f64x8::splat(2.0) * t118;
            acc_zk = tzk0;
            let t120 = t21 / t22;
            let t125 = f64x8::splat(1.0) / t23 / t53;
            let t126 = t33 * t125;
            let t127 = t30 * t126;
            let t128 = f64x8::splat(5.0) / f64x8::splat(27.0) * t127;
            let t135 = f64x8::splat(5.0) / f64x8::splat(4374.0) * t50 * t52 * t62;
            let t136 = t31 * t71;
            let t137 = t136 * v_lapl;
            let t139 = f64x8::splat(13.0) / f64x8::splat(7776.0) * t59 * t137;
            let t140 = t60 * t34;
            let t142 = f64x8::splat(1.0) / t22 / t140;
            let t145 = f64x8::splat(4.0) / f64x8::splat(6561.0) * t50 * t68 * t142;
            let t146 = -f64x8::splat(5.0) / f64x8::splat(243.0) * t127 - f64x8::splat(25.0) / f64x8::splat(162.0) * t30 * t41 * t36 - t135 + t139 - t145;
            let t149 = f64x8::splat(1.0) / t83 / t82;
            let t150 = t75 * t149;
            let t151 = t76 * t80;
            let t152 = -t135 + t139 - t145;
            let t155 = t79 * t78;
            let t156 = f64x8::splat(1.0) / t155;
            let t158 = t77 * t156 * t25;
            let t159 = t29 * v_sigma;
            let t160 = t32 * t125;
            let t161 = t159 * t160;
            let t164 = f64x8::splat(2.0) * t151 * t152 + f64x8::splat(10.0) / f64x8::splat(27.0) * t158 * t161;
            let t167 = t146 * t84 - t150 * t164 / f64x8::splat(2.0) + t128;
            let t169 = t95 * t95;
            let t170 = f64x8::splat(1.0) / t169;
            let t171 = t91 * t170;
            let t172 = ((t92).select(f64x8::splat(0.0), t167));
            let t173 = ((t94).select(t172, f64x8::splat(0.0)));
            let t174 = t173 * t98;
            let t175 = t104 * t110;
            let t176 = t174 * t175;
            let t178 = t105 * t91;
            let t179 = t99 * t99;
            let t180 = f64x8::splat(1.0) / t179;
            let t181 = t180 * t173;
            let t182 = f64x8::splat(1.0) / t103;
            let t184 = t102 * t182 * t110;
            let t187 = param_a * t170;
            let t188 = t173 * t107;
            let t190 = param_a * t180;
            let t191 = t173 * t102;
            let t193 = t187 * t188 - t190 * t191;
            let t195 = f64x8::splat(1.0) / t108;
            let t199 = ((t88).select(f64x8::splat(0.0), (t90).select(f64x8::splat(0.0), -t111 * param_b * t193 * t195 - t178 * t181 * t184 + t171 * t176)));
            let t201 = t167 * t112 + t86 * t199 - t128;
            let t206 = ((t3).select(f64x8::splat(0.0), t8 * t120 * t114 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t201));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t206 + f64x8::splat(2.0) * t118;
            acc_vrho = tvrho0;
            let t209 = t32 * t36;
            let t210 = t30 * t209;
            let t211 = f64x8::splat(5.0) / f64x8::splat(72.0) * t210;
            let t213 = t50 * t64;
            let t214 = t213 / f64x8::splat(2592.0);
            let t215 = v_sigma * t31;
            let t217 = t50 * t215 * t71;
            let t218 = t217 / f64x8::splat(4374.0);
            let t219 = f64x8::splat(5.0) / f64x8::splat(648.0) * t210 - t214 + t218;
            let t221 = -t214 + t218;
            let t224 = t29 * t32;
            let t225 = t224 * t36;
            let t228 = f64x8::splat(2.0) * t151 * t221 - f64x8::splat(5.0) / f64x8::splat(36.0) * t158 * t225;
            let t231 = t219 * t84 - t150 * t228 / f64x8::splat(2.0) - t211;
            let t233 = ((t92).select(f64x8::splat(0.0), t231));
            let t234 = ((t94).select(t233, f64x8::splat(0.0)));
            let t235 = t234 * t98;
            let t236 = t235 * t175;
            let t238 = t180 * t234;
            let t241 = t234 * t107;
            let t243 = t234 * t102;
            let t245 = t187 * t241 - t190 * t243;
            let t246 = param_b * t245;
            let t250 = ((t88).select(f64x8::splat(0.0), (t90).select(f64x8::splat(0.0), -t111 * t246 * t195 - t178 * t238 * t184 + t171 * t236)));
            let t252 = t231 * t112 + t86 * t250 + t211;
            let t256 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t252));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t256;
            acc_vsigma = tvsigma0;
            let t264 = t50 * v_lapl * t31 * t55 / f64x8::splat(1458.0);
            let t267 = t50 * t215 * t62 / f64x8::splat(2592.0);
            let t268 = f64x8::splat(5.0) / f64x8::splat(54.0) * t30 * t32 * t43 + t264 - t267;
            let t270 = t264 - t267;
            let t271 = t151 * t270;
            let t273 = -t150 * t271 + t268 * t84;
            let t275 = ((t92).select(f64x8::splat(0.0), t273));
            let t276 = ((t94).select(t275, f64x8::splat(0.0)));
            let t277 = t276 * t98;
            let t278 = t277 * t175;
            let t280 = t180 * t276;
            let t283 = t276 * t107;
            let t285 = t276 * t102;
            let t287 = t187 * t283 - t190 * t285;
            let t288 = param_b * t287;
            let t292 = ((t88).select(f64x8::splat(0.0), (t90).select(f64x8::splat(0.0), -t111 * t288 * t195 - t178 * t280 * t184 + t171 * t278)));
            let t294 = t273 * t112 + t86 * t292;
            let t298 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t294));
            let tvlapl0 = f64x8::splat(2.0) * v_rho * t298;
            acc_vlapl = tvlapl0;
            let tvtau0 = f64x8::splat(0.0);
            acc_vtau = tvtau0;
            let t303 = t21 / t22 / v_rho;
            let t311 = f64x8::splat(1.0) / t23 / t60;
            let t312 = t33 * t311;
            let t313 = t30 * t312;
            let t314 = f64x8::splat(55.0) / f64x8::splat(81.0) * t313;
            let t321 = f64x8::splat(65.0) / f64x8::splat(13122.0) * t50 * t52 * t71;
            let t322 = t31 * t142;
            let t323 = t322 * v_lapl;
            let t325 = f64x8::splat(13.0) / f64x8::splat(1458.0) * t59 * t323;
            let t326 = t60 * t53;
            let t328 = f64x8::splat(1.0) / t22 / t326;
            let t329 = t68 * t328;
            let t330 = t50 * t329;
            let t331 = f64x8::splat(76.0) / f64x8::splat(19683.0) * t330;
            let t332 = f64x8::splat(55.0) / f64x8::splat(729.0) * t313 + f64x8::splat(100.0) / f64x8::splat(243.0) * t30 * t41 * t125 + t321 - t325 + t331;
            let t334 = t146 * t149;
            let t336 = t82 * t82;
            let t338 = f64x8::splat(1.0) / t83 / t336;
            let t339 = t75 * t338;
            let t340 = t164 * t164;
            let t343 = t152 * t152;
            let t346 = t76 * t156;
            let t347 = t152 * t25;
            let t348 = t346 * t347;
            let t351 = t321 - t325 + t331;
            let t354 = t79 * t79;
            let t355 = f64x8::splat(1.0) / t354;
            let t356 = t77 * t355;
            let t357 = t356 * t47;
            let t358 = t49 * t67;
            let t359 = t31 * t328;
            let t360 = t358 * t359;
            let t363 = t32 * t311;
            let t364 = t159 * t363;
            let t367 = f64x8::splat(2.0) * t343 * t80 + f64x8::splat(40.0) / f64x8::splat(27.0) * t348 * t161 + f64x8::splat(2.0) * t151 * t351 + f64x8::splat(100.0) / f64x8::splat(243.0) * t357 * t360 - f64x8::splat(110.0) / f64x8::splat(81.0) * t158 * t364;
            let t370 = t332 * t84 - t334 * t164 + f64x8::splat(3.0) / f64x8::splat(4.0) * t339 * t340 - t150 * t367 / f64x8::splat(2.0) - t314;
            let t374 = t169 * t95;
            let t375 = f64x8::splat(1.0) / t374;
            let t376 = t91 * t375;
            let t377 = t173 * t173;
            let t378 = t377 * t98;
            let t379 = t378 * t175;
            let t382 = ((t92).select(f64x8::splat(0.0), t370));
            let t383 = ((t94).select(t382, f64x8::splat(0.0)));
            let t384 = t383 * t98;
            let t385 = t384 * t175;
            let t387 = param_a * param_a;
            let t388 = param_b * param_b;
            let t389 = t387 * t388;
            let t390 = t169 * t169;
            let t391 = f64x8::splat(1.0) / t390;
            let t392 = t389 * t391;
            let t395 = t170 * t377 * t98;
            let t396 = t389 * t395;
            let t397 = t104 * t180;
            let t398 = t397 * t184;
            let t401 = param_a * t388;
            let t402 = t170 * t173;
            let t403 = t401 * t402;
            let t405 = t110 * t193 * t195;
            let t406 = t105 * t405;
            let t409 = t105 * t389;
            let t410 = t179 * t179;
            let t411 = f64x8::splat(1.0) / t410;
            let t412 = t411 * t377;
            let t413 = t102 * t102;
            let t414 = t103 * t103;
            let t415 = f64x8::splat(1.0) / t414;
            let t416 = t413 * t415;
            let t417 = t416 * t110;
            let t418 = t412 * t417;
            let t420 = t179 * t99;
            let t421 = f64x8::splat(1.0) / t420;
            let t422 = t421 * t377;
            let t426 = t180 * t383;
            let t429 = param_b * t387;
            let t430 = t105 * t429;
            let t435 = t105 * t401 * t180;
            let t436 = t191 * t182;
            let t437 = t436 * t405;
            let t440 = t193 * t193;
            let t441 = t388 * t440;
            let t442 = t108 * t108;
            let t443 = f64x8::splat(1.0) / t442;
            let t446 = param_a * t375;
            let t447 = t377 * t107;
            let t452 = t387 * t391;
            let t454 = param_a * t421;
            let t455 = t377 * t102;
            let t458 = t383 * t102;
            let t460 = t387 * t411;
            let t462 = t187 * t383 * t107 - t190 * t458 - f64x8::splat(2.0) * t446 * t447 + t452 * t447 - f64x8::splat(2.0) * t454 * t455 + t460 * t455;
            let t463 = param_b * t462;
            let t469 = t111 * param_b * t440 * t443 - t111 * t463 * t195 + t111 * t441 * t443 - f64x8::splat(2.0) * t178 * t422 * t184 - t178 * t426 * t184 + t430 * t412 * t184 + t171 * t385 - f64x8::splat(2.0) * t376 * t379 + t392 * t379 - f64x8::splat(2.0) * t396 * t398 - f64x8::splat(2.0) * t403 * t406 + t409 * t418 - t430 * t418 + f64x8::splat(2.0) * t435 * t437;
            let t470 = ((t88).select(f64x8::splat(0.0), (t90).select(f64x8::splat(0.0), t469)));
            let t472 = t370 * t112 + f64x8::splat(2.0) * t167 * t199 + t86 * t470 + t314;
            let t477 = ((t3).select(f64x8::splat(0.0), -t8 * t303 * t114 / f64x8::splat(30.0) + t8 * t120 * t201 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t472));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t477 + f64x8::splat(4.0) * t206;
            acc_v2rho2 = tv2rho20;
            let t483 = t30 * t160;
            let t484 = f64x8::splat(5.0) / f64x8::splat(27.0) * t483;
            let t486 = t50 * t137;
            let t487 = f64x8::splat(13.0) / f64x8::splat(7776.0) * t486;
            let t488 = t215 * t142;
            let t489 = t50 * t488;
            let t490 = f64x8::splat(8.0) / f64x8::splat(6561.0) * t489;
            let t491 = -f64x8::splat(5.0) / f64x8::splat(243.0) * t483 + t487 - t490;
            let t493 = t219 * t149;
            let t498 = t228 * t164;
            let t501 = t152 * t80;
            let t504 = t221 * t25;
            let t505 = t346 * t504;
            let t508 = t487 - t490;
            let t511 = t346 * t25;
            let t516 = t49 * t31;
            let t517 = t142 * v_sigma;
            let t518 = t516 * t517;
            let t521 = t224 * t125;
            let t524 = f64x8::splat(2.0) * t501 * t221 + f64x8::splat(20.0) / f64x8::splat(27.0) * t505 * t161 + f64x8::splat(2.0) * t151 * t508 - f64x8::splat(5.0) / f64x8::splat(18.0) * t511 * t224 * t36 * t152 - f64x8::splat(25.0) / f64x8::splat(162.0) * t357 * t518 + f64x8::splat(10.0) / f64x8::splat(27.0) * t158 * t521;
            let t527 = t491 * t84 - t493 * t164 / f64x8::splat(2.0) - t334 * t228 / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t339 * t498 - t150 * t524 / f64x8::splat(2.0) + t484;
            let t531 = t375 * t234;
            let t532 = t91 * t531;
            let t535 = ((t92).select(f64x8::splat(0.0), t527));
            let t536 = ((t94).select(t535, f64x8::splat(0.0)));
            let t537 = t536 * t98;
            let t538 = t537 * t175;
            let t540 = t391 * t234;
            let t541 = t389 * t540;
            let t543 = t170 * t234;
            let t544 = t543 * t98;
            let t545 = t389 * t544;
            let t547 = t397 * t173 * t184;
            let t550 = t401 * t543;
            let t553 = t105 * t389 * t411;
            let t554 = t173 * t413;
            let t556 = t415 * t234 * t110;
            let t557 = t554 * t556;
            let t560 = t105 * t91 * t421;
            let t561 = t182 * t110;
            let t562 = t561 * t173;
            let t563 = t243 * t562;
            let t566 = t180 * t536;
            let t570 = t105 * t429 * t411;
            let t573 = t243 * t182;
            let t574 = t573 * t405;
            let t576 = t110 * t245;
            let t577 = t576 * t195;
            let t578 = t105 * t577;
            let t580 = t436 * t577;
            let t582 = t388 * t245;
            let t583 = t443 * t193;
            let t586 = t241 * t173;
            let t589 = t536 * t107;
            let t592 = t243 * t173;
            let t595 = t536 * t102;
            let t598 = t187 * t589 - t190 * t595 - f64x8::splat(2.0) * t446 * t586 + t452 * t586 - f64x8::splat(2.0) * t454 * t592 + t460 * t592;
            let t599 = param_b * t598;
            let t604 = -t111 * t599 * t195 + t111 * t246 * t583 + t111 * t582 * t583 - t178 * t566 * t184 + t171 * t538 - f64x8::splat(2.0) * t532 * t176 + t541 * t176 - t403 * t578 - t550 * t406 + t435 * t574 + t435 * t580 - f64x8::splat(2.0) * t545 * t547 + t553 * t557 - t570 * t557 - f64x8::splat(2.0) * t560 * t563 + t570 * t563;
            let t605 = ((t88).select(f64x8::splat(0.0), (t90).select(f64x8::splat(0.0), t604)));
            let t607 = t527 * t112 + t167 * t250 + t231 * t199 + t86 * t605 - t484;
            let t612 = ((t3).select(f64x8::splat(0.0), t8 * t120 * t252 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t607));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t612 + f64x8::splat(2.0) * t256;
            acc_v2rhosigma = tv2rhosigma0;
            let t619 = f64x8::splat(5.0) / f64x8::splat(2187.0) * t213;
            let t620 = f64x8::splat(13.0) / f64x8::splat(7776.0) * t217;
            let t621 = -f64x8::splat(25.0) / f64x8::splat(162.0) * t210 - t619 + t620;
            let t623 = t268 * t149;
            let t627 = t339 * t76;
            let t628 = t80 * t270;
            let t629 = t628 * t164;
            let t632 = t501 * t270;
            let t634 = t346 * t270;
            let t635 = t150 * t634;
            let t638 = -t619 + t620;
            let t639 = t151 * t638;
            let t641 = t621 * t84 - t623 * t164 / f64x8::splat(2.0) - t334 * t271 + f64x8::splat(3.0) / f64x8::splat(2.0) * t627 * t629 - t150 * t632 - f64x8::splat(10.0) / f64x8::splat(27.0) * t635 * t127 - t150 * t639;
            let t645 = t375 * t276;
            let t646 = t91 * t645;
            let t649 = ((t92).select(f64x8::splat(0.0), t641));
            let t650 = ((t94).select(t649, f64x8::splat(0.0)));
            let t651 = t650 * t98;
            let t652 = t651 * t175;
            let t654 = t391 * t276;
            let t655 = t389 * t654;
            let t657 = t170 * t276;
            let t658 = t657 * t98;
            let t659 = t389 * t658;
            let t662 = t401 * t657;
            let t665 = t415 * t276 * t110;
            let t666 = t554 * t665;
            let t668 = t285 * t562;
            let t671 = t180 * t650;
            let t676 = t285 * t182;
            let t677 = t676 * t405;
            let t679 = t110 * t287;
            let t680 = t679 * t195;
            let t681 = t105 * t680;
            let t683 = t436 * t680;
            let t685 = t388 * t287;
            let t688 = t283 * t173;
            let t691 = t650 * t107;
            let t694 = t285 * t173;
            let t697 = t650 * t102;
            let t700 = t187 * t691 - t190 * t697 - f64x8::splat(2.0) * t446 * t688 + t452 * t688 - f64x8::splat(2.0) * t454 * t694 + t460 * t694;
            let t701 = param_b * t700;
            let t706 = -t111 * t701 * t195 + t111 * t288 * t583 + t111 * t685 * t583 - t178 * t671 * t184 + t171 * t652 - f64x8::splat(2.0) * t646 * t176 + t655 * t176 - t403 * t681 - t662 * t406 + t435 * t677 + t435 * t683 - f64x8::splat(2.0) * t659 * t547 + t553 * t666 - f64x8::splat(2.0) * t560 * t668 - t570 * t666 + t570 * t668;
            let t707 = ((t88).select(f64x8::splat(0.0), (t90).select(f64x8::splat(0.0), t706)));
            let t709 = t641 * t112 + t167 * t292 + t273 * t199 + t86 * t707;
            let t714 = ((t3).select(f64x8::splat(0.0), t8 * t120 * t294 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t709));
            let tv2rholapl0 = f64x8::splat(2.0) * v_rho * t714 + f64x8::splat(2.0) * t298;
            acc_v2rholapl = tv2rholapl0;
            let tv2rhotau0 = f64x8::splat(0.0);
            acc_v2rhotau = tv2rhotau0;
            let t718 = t50 * t136 * t84;
            let t721 = t228 * t228;
            let t724 = t221 * t221;
            let t727 = t346 * t221;
            let t730 = t151 * t47;
            let t731 = t516 * t71;
            let t736 = f64x8::splat(2.0) * t724 * t80 - f64x8::splat(5.0) / f64x8::splat(9.0) * t727 * t210 + t730 * t731 / f64x8::splat(2187.0) + f64x8::splat(25.0) / f64x8::splat(432.0) * t357 * t731;
            let t739 = t718 / f64x8::splat(4374.0) - t493 * t228 + f64x8::splat(3.0) / f64x8::splat(4.0) * t339 * t721 - t150 * t736 / f64x8::splat(2.0);
            let t743 = t234 * t234;
            let t744 = t743 * t98;
            let t745 = t744 * t175;
            let t748 = ((t92).select(f64x8::splat(0.0), t739));
            let t749 = ((t94).select(t748, f64x8::splat(0.0)));
            let t750 = t749 * t98;
            let t751 = t750 * t175;
            let t755 = t170 * t743 * t98;
            let t756 = t389 * t755;
            let t761 = t411 * t743;
            let t762 = t761 * t417;
            let t764 = t421 * t743;
            let t768 = t180 * t749;
            let t774 = t573 * t577;
            let t777 = t245 * t245;
            let t778 = t388 * t777;
            let t781 = t743 * t107;
            let t784 = t749 * t107;
            let t787 = t743 * t102;
            let t790 = t749 * t102;
            let t793 = t187 * t784 - t190 * t790 - f64x8::splat(2.0) * t446 * t781 + t452 * t781 - f64x8::splat(2.0) * t454 * t787 + t460 * t787;
            let t794 = param_b * t793;
            let t797 = param_b * t777;
            let t800 = -t111 * t794 * t195 + t111 * t778 * t443 + t111 * t797 * t443 - f64x8::splat(2.0) * t178 * t764 * t184 - t178 * t768 * t184 + t430 * t761 * t184 + t171 * t751 - f64x8::splat(2.0) * t376 * t745 + t392 * t745 - f64x8::splat(2.0) * t756 * t398 + t409 * t762 - t430 * t762 + f64x8::splat(2.0) * t435 * t774 - f64x8::splat(2.0) * t550 * t578;
            let t801 = ((t88).select(f64x8::splat(0.0), (t90).select(f64x8::splat(0.0), t800)));
            let t803 = t739 * t112 + f64x8::splat(2.0) * t231 * t250 + t86 * t801;
            let t807 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t803));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t807;
            acc_v2sigma2 = tv2sigma20;
            let t810 = t50 * t63 * t84;
            let t815 = t628 * t228;
            let t818 = t221 * t80;
            let t819 = t818 * t270;
            let t821 = t150 * t346;
            let t822 = t270 * t25;
            let t823 = t822 * t225;
            let t826 = t150 * t151;
            let t827 = t50 * t63;
            let t828 = t826 * t827;
            let t830 = -t810 / f64x8::splat(2592.0) - t623 * t228 / f64x8::splat(2.0) - t493 * t271 + f64x8::splat(3.0) / f64x8::splat(2.0) * t627 * t815 - t150 * t819 + f64x8::splat(5.0) / f64x8::splat(36.0) * t821 * t823 + t828 / f64x8::splat(2592.0);
            let t836 = ((t92).select(f64x8::splat(0.0), t830));
            let t837 = ((t94).select(t836, f64x8::splat(0.0)));
            let t838 = t837 * t98;
            let t839 = t838 * t175;
            let t843 = t397 * t234 * t184;
            let t847 = t234 * t413;
            let t848 = t847 * t665;
            let t850 = t561 * t234;
            let t851 = t285 * t850;
            let t854 = t180 * t837;
            let t859 = t676 * t577;
            let t862 = t573 * t680;
            let t864 = t443 * t245;
            let t867 = t283 * t234;
            let t870 = t837 * t107;
            let t873 = t285 * t234;
            let t876 = t837 * t102;
            let t879 = t187 * t870 - t190 * t876 - f64x8::splat(2.0) * t446 * t867 + t452 * t867 - f64x8::splat(2.0) * t454 * t873 + t460 * t873;
            let t880 = param_b * t879;
            let t885 = -t111 * t880 * t195 + t111 * t288 * t864 + t111 * t685 * t864 - t178 * t854 * t184 + t171 * t839 - f64x8::splat(2.0) * t646 * t236 + t655 * t236 + t435 * t859 + t435 * t862 - t550 * t681 + t553 * t848 - f64x8::splat(2.0) * t560 * t851 - t570 * t848 + t570 * t851 - t662 * t578 - f64x8::splat(2.0) * t659 * t843;
            let t886 = ((t88).select(f64x8::splat(0.0), (t90).select(f64x8::splat(0.0), t885)));
            let t888 = t830 * t112 + t231 * t292 + t273 * t250 + t86 * t886;
            let t892 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t888));
            let tv2sigmalapl0 = f64x8::splat(2.0) * v_rho * t892;
            acc_v2sigmalapl = tv2sigmalapl0;
            let tv2sigmatau0 = f64x8::splat(0.0);
            acc_v2sigmatau = tv2sigmatau0;
            let t894 = t31 * t55;
            let t900 = t270 * t270;
            let t901 = t356 * t900;
            let t904 = t900 * t80;
            let t906 = t50 * t894;
            let t909 = t50 * t894 * t84 / f64x8::splat(1458.0) - f64x8::splat(2.0) * t623 * t271 + f64x8::splat(3.0) * t339 * t901 - t150 * t904 - t826 * t906 / f64x8::splat(1458.0);
            let t913 = t276 * t276;
            let t914 = t913 * t98;
            let t915 = t914 * t175;
            let t918 = ((t92).select(f64x8::splat(0.0), t909));
            let t919 = ((t94).select(t918, f64x8::splat(0.0)));
            let t920 = t919 * t98;
            let t921 = t920 * t175;
            let t925 = t170 * t913 * t98;
            let t926 = t389 * t925;
            let t931 = t411 * t913;
            let t932 = t931 * t417;
            let t934 = t421 * t913;
            let t944 = t676 * t680;
            let t947 = t287 * t287;
            let t948 = t388 * t947;
            let t951 = t913 * t107;
            let t954 = t919 * t107;
            let t957 = t913 * t102;
            let t960 = t919 * t102;
            let t963 = t187 * t954 - t190 * t960 - f64x8::splat(2.0) * t446 * t951 + t452 * t951 - f64x8::splat(2.0) * t454 * t957 + t460 * t957;
            let t964 = param_b * t963;
            let t967 = param_b * t947;
            let t970 = -t178 * t180 * t919 * t184 - t111 * t964 * t195 + t111 * t948 * t443 + t111 * t967 * t443 - f64x8::splat(2.0) * t178 * t934 * t184 + t430 * t931 * t184 + t171 * t921 - f64x8::splat(2.0) * t376 * t915 + t392 * t915 - f64x8::splat(2.0) * t926 * t398 + t409 * t932 - t430 * t932 + f64x8::splat(2.0) * t435 * t944 - f64x8::splat(2.0) * t662 * t681;
            let t971 = ((t88).select(f64x8::splat(0.0), (t90).select(f64x8::splat(0.0), t970)));
            let t973 = t909 * t112 + f64x8::splat(2.0) * t273 * t292 + t86 * t971;
            let t977 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t973));
            let tv2lapl20 = f64x8::splat(2.0) * v_rho * t977;
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let tv2tau20 = f64x8::splat(0.0);
            acc_v2tau2 = tv2tau20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rholapl.into(); v2rholapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhotau.into(); v2rhotau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigmalapl.into(); v2sigmalapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigmatau.into(); v2sigmatau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2lapl2.into(); v2lapl2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2lapltau.into(); v2lapltau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2tau2.into(); v2tau2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
