//! GGA_C_ZVPBELOC fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_zvpbeloc.c`
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
pub fn gga_c_zvpbeloc_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
        {
            let t1 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t2 = t1 * t1;
            let t3 = t2 * t2;
            let t4 = t3 * t1;
            let t5 = (simd::pow(f64x8::splat(3.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t6 = t4 * t5;
            let t7 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = (simd::pow(t8, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t10 = t6 * t9;
            let t11 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t12 = (simd::cbrt(t11));
            let t13 = (simd::cbrt(v_rho));
            let t14 = f64x8::splat(1.0) / t13;
            let t17 = (((f64x8::splat(1e-20)).simd_lt(f64x8::splat(0.0))).select(f64x8::splat(0.0), f64x8::splat(1e-20)));
            let t19 = t10 * t12 * t14 * t17;
            let t21 = (simd::exp(-f64x8::splat(1.0) * t19));
            let t22 = f64x8::splat(M_CBRT3);
            let t23 = t22 * t12;
            let t24 = f64x8::splat(M_CBRT4);
            let t25 = t24 * t24;
            let t27 = t23 * t25 * t14;
            let t29 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t27;
            let t30 = ((t27).sqrt());
            let t33 = ((t27) * (t27).sqrt());
            let t35 = t22 * t22;
            let t36 = t12 * t12;
            let t37 = t35 * t36;
            let t38 = t13 * t13;
            let t41 = t37 * t24 / t38;
            let t43 = f64x8::splat(3.79785) * t30 + f64x8::splat(0.8969) * t27 + f64x8::splat(0.204775) * t33 + f64x8::splat(0.123235) * t41;
            let t46 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t43;
            let t47 = (simd::ln(t46));
            let t49 = f64x8::splat(0.0621814) * t29 * t47;
            let t50 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t51 = (simd::cbrt(zeta_threshold));
            let t53 = ((t50).select(t51 * zeta_threshold, f64x8::splat(1.0)));
            let t56 = f64x8::splat(M_CBRT2);
            let t60 = (f64x8::splat(2.0) * t53 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t56 - f64x8::splat(2.0));
            let t62 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t27;
            let t67 = f64x8::splat(5.1785) * t30 + f64x8::splat(0.905775) * t27 + f64x8::splat(0.1100325) * t33 + f64x8::splat(0.1241775) * t41;
            let t70 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t67;
            let t71 = (simd::ln(t70));
            let t74 = f64x8::splat(0.0197516734986138) * t60 * t62 * t71;
            let t75 = (simd::ln(f64x8::splat(2.0)));
            let t76 = f64x8::splat(1.0) - t75;
            let t77 = t76 * t8;
            let t78 = t51 * t51;
            let t79 = ((t50).select(t78, f64x8::splat(1.0)));
            let t80 = t79 * t79;
            let t81 = t80 * t79;
            let t82 = v_rho * v_rho;
            let t84 = f64x8::splat(1.0) / t13 / t82;
            let t85 = v_sigma * t84;
            let t86 = f64x8::splat(1.0) / t80;
            let t87 = t56 * t86;
            let t89 = f64x8::splat(1.0) / t12;
            let t90 = t35 * t89;
            let t92 = (simd::exp(-t41 / f64x8::splat(4.0)));
            let t93 = f64x8::splat(1.0) - t92;
            let t94 = t24 * t93;
            let t95 = t90 * t94;
            let t98 = f64x8::splat(0.0375) + f64x8::splat(0.0008333333333333334) * t85 * t87 * t95;
            let t100 = t86 * t35;
            let t102 = t100 * t89 * t24;
            let t105 = f64x8::splat(1.0) / t76;
            let t106 = t98 * t105;
            let t109 = f64x8::splat(1.0) / t81;
            let t112 = (simd::exp(-(-t49 + t74) * t105 * t7 * t109));
            let t113 = t112 - f64x8::splat(1.0);
            let t114 = f64x8::splat(1.0) / t113;
            let t115 = t7 * t114;
            let t116 = v_sigma * v_sigma;
            let t117 = t115 * t116;
            let t118 = t106 * t117;
            let t119 = t82 * t82;
            let t121 = f64x8::splat(1.0) / t38 / t119;
            let t122 = t56 * t56;
            let t123 = t121 * t122;
            let t124 = t80 * t80;
            let t125 = f64x8::splat(1.0) / t124;
            let t127 = f64x8::splat(1.0) / t36;
            let t129 = t22 * t127 * t25;
            let t130 = t123 * t125 * t129;
            let t133 = t85 * t56 * t102 / f64x8::splat(96.0) + t118 * t130 / f64x8::splat(3072.0);
            let t134 = t98 * t133;
            let t135 = t105 * t7;
            let t136 = t115 * t133;
            let t138 = t106 * t136 + f64x8::splat(1.0);
            let t139 = f64x8::splat(1.0) / t138;
            let t140 = t135 * t139;
            let t142 = t134 * t140 + f64x8::splat(1.0);
            let t143 = (simd::ln(t142));
            let tzk0 = t21 * (t77 * t81 * t143 - t49 + t74);
            acc_zk = tzk0;
            let t147 = t14 * t4;
            let t148 = t5 * t9;
            let t149 = t147 * t148;
            let t150 = t12 * t17;
            let t154 = v_rho * t21;
            let t156 = f64x8::splat(1.0) / t13 / v_rho;
            let t157 = t25 * t156;
            let t160 = f64x8::splat(0.0011073470983333333) * t23 * t157 * t47;
            let t161 = t43 * t43;
            let t162 = f64x8::splat(1.0) / t161;
            let t163 = t29 * t162;
            let t165 = f64x8::splat(1.0) / t30 * t22;
            let t166 = t12 * t25;
            let t167 = t166 * t156;
            let t168 = t165 * t167;
            let t170 = t23 * t157;
            let t172 = ((t27).sqrt());
            let t173 = t172 * t22;
            let t174 = t173 * t167;
            let t177 = f64x8::splat(1.0) / t38 / v_rho;
            let t179 = t37 * t24 * t177;
            let t181 = -f64x8::splat(0.632975) * t168 - f64x8::splat(0.29896666666666666) * t170 - f64x8::splat(0.1023875) * t174 - f64x8::splat(0.08215666666666667) * t179;
            let t182 = f64x8::splat(1.0) / t46;
            let t183 = t181 * t182;
            let t185 = f64x8::splat(1.0) * t163 * t183;
            let t186 = t60 * t22;
            let t190 = f64x8::splat(0.00018311447306006544) * t186 * t166 * t156 * t71;
            let t191 = t60 * t62;
            let t192 = t67 * t67;
            let t193 = f64x8::splat(1.0) / t192;
            let t198 = -f64x8::splat(0.8630833333333333) * t168 - f64x8::splat(0.301925) * t170 - f64x8::splat(0.05501625) * t174 - f64x8::splat(0.082785) * t179;
            let t200 = f64x8::splat(1.0) / t70;
            let t201 = t193 * t198 * t200;
            let t203 = f64x8::splat(0.5848223622634646) * t191 * t201;
            let t204 = t82 * v_rho;
            let t206 = f64x8::splat(1.0) / t13 / t204;
            let t207 = v_sigma * t206;
            let t211 = f64x8::splat(1.0) / t119;
            let t214 = t25 * t92;
            let t215 = t23 * t214;
            let t218 = -f64x8::splat(0.0019444444444444444) * t207 * t87 * t95 - f64x8::splat(0.0004166666666666667) * v_sigma * t211 * t87 * t215;
            let t219 = t218 * t133;
            let t224 = t218 * t105;
            let t225 = t224 * t117;
            let t228 = t76 * t76;
            let t229 = f64x8::splat(1.0) / t228;
            let t230 = t98 * t229;
            let t231 = t7 * t7;
            let t232 = t230 * t231;
            let t233 = t113 * t113;
            let t234 = f64x8::splat(1.0) / t233;
            let t235 = t234 * t116;
            let t236 = t235 * t121;
            let t237 = t232 * t236;
            let t238 = t124 * t81;
            let t239 = f64x8::splat(1.0) / t238;
            let t241 = t122 * t239 * t22;
            let t242 = t127 * t25;
            let t243 = t160 + t185 - t190 - t203;
            let t244 = t243 * t112;
            let t246 = t241 * t242 * t244;
            let t249 = t119 * v_rho;
            let t251 = f64x8::splat(1.0) / t38 / t249;
            let t252 = t251 * t122;
            let t254 = t252 * t125 * t129;
            let t257 = -f64x8::splat(7.0) / f64x8::splat(288.0) * t207 * t56 * t102 + t225 * t130 / f64x8::splat(3072.0) + t237 * t246 / f64x8::splat(3072.0) - f64x8::splat(7.0) / f64x8::splat(4608.0) * t118 * t254;
            let t258 = t98 * t257;
            let t260 = t134 * t105;
            let t261 = t138 * t138;
            let t262 = f64x8::splat(1.0) / t261;
            let t263 = t7 * t262;
            let t265 = t231 * t234;
            let t266 = t230 * t265;
            let t267 = t133 * t243;
            let t268 = t109 * t112;
            let t269 = t267 * t268;
            let t271 = t115 * t257;
            let t273 = t106 * t271 + t224 * t136 + t266 * t269;
            let t274 = t263 * t273;
            let t276 = t219 * t140 + t258 * t140 - t260 * t274;
            let t278 = f64x8::splat(1.0) / t142;
            let t281 = t77 * t81 * t276 * t278 + t160 + t185 - t190 - t203;
            let tvrho0 = tzk0 + f64x8::splat(0.3333333333333333) * t149 * t150 * tzk0 + t154 * t281;
            acc_vrho = tvrho0;
            let t283 = t154 * t76;
            let t284 = t8 * t81;
            let t285 = t84 * t56;
            let t286 = t100 * t89;
            let t287 = t285 * t286;
            let t289 = t133 * t105 * t139;
            let t290 = t94 * t289;
            let t294 = t90 * t24;
            let t295 = t285 * t86 * t294;
            let t297 = t119 * t204;
            let t298 = f64x8::splat(1.0) / t297;
            let t300 = f64x8::splat(1.0) / t124 / t80;
            let t302 = t298 * t300 * t93;
            let t303 = t105 * t114;
            let t304 = t303 * t116;
            let t307 = t115 * v_sigma;
            let t308 = t106 * t307;
            let t311 = t295 / f64x8::splat(96.0) + f64x8::splat(0.00020186378047070194) * t302 * t304 + t308 * t130 / f64x8::splat(1536.0);
            let t312 = t98 * t311;
            let t314 = t303 * t133;
            let t315 = t94 * t314;
            let t318 = t115 * t311;
            let t320 = f64x8::splat(0.008224670334241133) * t287 * t315 + t106 * t318;
            let t321 = t263 * t320;
            let t323 = f64x8::splat(0.008224670334241133) * t287 * t290 + t312 * t140 - t260 * t321;
            let tvsigma0 = t283 * t284 * t323 * t278;
            acc_vsigma = tvsigma0;
            let t326 = t9 * t12;
            let t327 = t6 * t326;
            let t328 = t156 * t17;
            let t332 = t21 * t281;
            let t334 = t177 * t25;
            let t335 = (simd::cbrt(t8));
            let t336 = t22 * t335;
            let t337 = t334 * t336;
            let t338 = t17 * t17;
            let t339 = t36 * t338;
            let t346 = t25 * t84;
            let t349 = f64x8::splat(0.0014764627977777779) * t23 * t346 * t47;
            let t350 = t23 * t25;
            let t351 = t156 * t162;
            let t354 = f64x8::splat(0.035616666666666665) * t350 * t351 * t183;
            let t355 = t161 * t43;
            let t356 = f64x8::splat(1.0) / t355;
            let t357 = t29 * t356;
            let t358 = t181 * t181;
            let t359 = t358 * t182;
            let t361 = f64x8::splat(2.0) * t357 * t359;
            let t364 = f64x8::splat(1.0) / t30 / t27 * t35;
            let t365 = t36 * t24;
            let t367 = f64x8::splat(1.0) / t38 / t82;
            let t368 = t365 * t367;
            let t369 = t364 * t368;
            let t371 = t166 * t84;
            let t372 = t165 * t371;
            let t374 = t23 * t346;
            let t376 = f64x8::splat(1.0)/((t27).sqrt());
            let t377 = t376 * t35;
            let t378 = t377 * t368;
            let t380 = t173 * t371;
            let t383 = t37 * t24 * t367;
            let t385 = -f64x8::splat(0.4219833333333333) * t369 + f64x8::splat(0.8439666666666666) * t372 + f64x8::splat(0.3986222222222222) * t374 + f64x8::splat(0.06825833333333334) * t378 + f64x8::splat(0.13651666666666668) * t380 + f64x8::splat(0.1369277777777778) * t383;
            let t386 = t385 * t182;
            let t388 = f64x8::splat(1.0) * t163 * t386;
            let t389 = t161 * t161;
            let t390 = f64x8::splat(1.0) / t389;
            let t391 = t29 * t390;
            let t392 = t46 * t46;
            let t393 = f64x8::splat(1.0) / t392;
            let t394 = t358 * t393;
            let t396 = f64x8::splat(16.081979498692537) * t391 * t394;
            let t400 = f64x8::splat(0.00024415263074675396) * t186 * t166 * t84 * t71;
            let t401 = t60 * t23;
            let t404 = f64x8::splat(0.01084358130030174) * t401 * t157 * t201;
            let t405 = t192 * t67;
            let t406 = f64x8::splat(1.0) / t405;
            let t407 = t198 * t198;
            let t409 = t406 * t407 * t200;
            let t411 = f64x8::splat(1.1696447245269292) * t191 * t409;
            let t418 = -f64x8::splat(0.5753888888888888) * t369 + f64x8::splat(1.1507777777777777) * t372 + f64x8::splat(0.4025666666666667) * t374 + f64x8::splat(0.0366775) * t378 + f64x8::splat(0.073355) * t380 + f64x8::splat(0.137975) * t383;
            let t420 = t193 * t418 * t200;
            let t422 = f64x8::splat(0.5848223622634646) * t191 * t420;
            let t423 = t192 * t192;
            let t424 = f64x8::splat(1.0) / t423;
            let t425 = t424 * t407;
            let t426 = t70 * t70;
            let t427 = f64x8::splat(1.0) / t426;
            let t428 = t425 * t427;
            let t430 = f64x8::splat(17.315859105681465) * t191 * t428;
            let t432 = f64x8::splat(1.0) / t13 / t119;
            let t433 = v_sigma * t432;
            let t437 = f64x8::splat(1.0) / t249;
            let t444 = t86 * t11;
            let t445 = t444 * t92;
            let t448 = f64x8::splat(0.006481481481481481) * t433 * t87 * t95 + f64x8::splat(0.002638888888888889) * v_sigma * t437 * t87 * t215 - f64x8::splat(0.0008333333333333334) * v_sigma * t251 * t56 * t445;
            let t449 = t448 * t133;
            let t451 = t218 * t257;
            let t454 = t219 * t105;
            let t460 = t448 * t105;
            let t461 = t460 * t117;
            let t464 = t218 * t229;
            let t465 = t464 * t231;
            let t466 = t465 * t236;
            let t472 = f64x8::splat(1.0) / t228 / t76;
            let t473 = t98 * t472;
            let t474 = t231 * t7;
            let t475 = t473 * t474;
            let t477 = f64x8::splat(1.0) / t233 / t113;
            let t478 = t477 * t116;
            let t479 = t478 * t121;
            let t480 = t475 * t479;
            let t481 = t124 * t124;
            let t483 = f64x8::splat(1.0) / t481 / t80;
            let t485 = t122 * t483 * t22;
            let t486 = t243 * t243;
            let t487 = t112 * t112;
            let t490 = t485 * t242 * t486 * t487;
            let t493 = t235 * t251;
            let t494 = t232 * t493;
            let t497 = -t349 - t354 - t361 + t388 + t396 + t400 + t404 + t411 - t422 - t430;
            let t498 = t497 * t112;
            let t500 = t241 * t242 * t498;
            let t503 = t475 * t236;
            let t506 = t485 * t242 * t486 * t112;
            let t509 = t119 * t82;
            let t511 = f64x8::splat(1.0) / t38 / t509;
            let t514 = t511 * t122 * t125 * t129;
            let t517 = f64x8::splat(35.0) / f64x8::splat(432.0) * t433 * t56 * t102 + t461 * t130 / f64x8::splat(3072.0) + t466 * t246 / f64x8::splat(1536.0) - f64x8::splat(7.0) / f64x8::splat(2304.0) * t225 * t254 + t480 * t490 / f64x8::splat(1536.0) - f64x8::splat(7.0) / f64x8::splat(2304.0) * t494 * t246 + t237 * t500 / f64x8::splat(3072.0) - t503 * t506 / f64x8::splat(3072.0) + f64x8::splat(119.0) / f64x8::splat(13824.0) * t118 * t514;
            let t518 = t98 * t517;
            let t520 = t258 * t105;
            let t524 = f64x8::splat(1.0) / t261 / t138;
            let t525 = t7 * t524;
            let t526 = t273 * t273;
            let t527 = t525 * t526;
            let t531 = t464 * t265;
            let t536 = t474 * t477;
            let t537 = t473 * t536;
            let t538 = t133 * t486;
            let t539 = t300 * t487;
            let t540 = t538 * t539;
            let t543 = t257 * t243;
            let t544 = t543 * t268;
            let t547 = t133 * t497;
            let t548 = t547 * t268;
            let t550 = t474 * t234;
            let t551 = t473 * t550;
            let t552 = t300 * t112;
            let t553 = t538 * t552;
            let t555 = t115 * t517;
            let t557 = t106 * t555 + t460 * t136 + f64x8::splat(2.0) * t224 * t271 + f64x8::splat(2.0) * t266 * t544 + t266 * t548 + f64x8::splat(2.0) * t531 * t269 + f64x8::splat(2.0) * t537 * t540 - t551 * t553;
            let t558 = t263 * t557;
            let t560 = t449 * t140 + f64x8::splat(2.0) * t451 * t140 + t518 * t140 + f64x8::splat(2.0) * t260 * t527 - t260 * t558 - f64x8::splat(2.0) * t454 * t274 - f64x8::splat(2.0) * t520 * t274;
            let t564 = t276 * t276;
            let t566 = t142 * t142;
            let t567 = f64x8::splat(1.0) / t566;
            let t570 = t77 * t81 * t560 * t278 - t77 * t81 * t564 * t567 - t349 - t354 - t361 + t388 + t396 + t400 + t404 + t411 - t422 - t430;
            let tv2rho20 = f64x8::splat(0.2222222222222222) * t327 * t328 * tzk0 + f64x8::splat(2.0) * t332 + f64x8::splat(0.4444444444444444) * t337 * t339 * tzk0 + f64x8::splat(0.6666666666666666) * t149 * t150 * t332 + t154 * t570;
            acc_v2rho2 = tv2rho20;
            let t572 = t21 * t76;
            let t573 = t572 * t8;
            let t574 = t81 * t323;
            let t575 = t574 * t278;
            let t578 = t147 * t148 * t12;
            let t580 = t17 * t21 * t76;
            let t581 = t580 * t575;
            let t584 = t206 * t56;
            let t585 = t584 * t286;
            let t588 = t211 * t56;
            let t590 = t86 * t22 * t12;
            let t591 = t588 * t590;
            let t592 = t214 * t289;
            let t596 = t257 * t105 * t139;
            let t597 = t94 * t596;
            let t600 = t94 * t133;
            let t601 = t105 * t262;
            let t602 = t601 * t273;
            let t603 = t600 * t602;
            let t606 = t218 * t311;
            let t609 = t584 * t86 * t294;
            let t611 = t119 * t119;
            let t612 = f64x8::splat(1.0) / t611;
            let t614 = t612 * t300 * t93;
            let t618 = f64x8::splat(1.0) / t38 / t611;
            let t620 = t618 * t300 * t37;
            let t621 = t24 * t92;
            let t622 = t621 * t304;
            let t626 = f64x8::splat(1.0) / t481 / t79;
            let t628 = t93 * t229;
            let t629 = t298 * t626 * t628;
            let t631 = t243 * t7 * t112;
            let t632 = t235 * t631;
            let t635 = t224 * t307;
            let t638 = t234 * v_sigma;
            let t639 = t638 * t121;
            let t640 = t232 * t639;
            let t645 = -f64x8::splat(7.0) / f64x8::splat(288.0) * t609 - f64x8::splat(0.0014130464632949138) * t614 * t304 - f64x8::splat(3.364396341178366e-05) * t620 * t622 + f64x8::splat(0.00020186378047070194) * t629 * t632 + t635 * t130 / f64x8::splat(1536.0) + t640 * t246 / f64x8::splat(1536.0) - f64x8::splat(7.0) / f64x8::splat(2304.0) * t308 * t254;
            let t646 = t98 * t645;
            let t648 = t312 * t105;
            let t653 = t525 * t320 * t273;
            let t658 = t214 * t314;
            let t661 = t124 * t79;
            let t662 = f64x8::splat(1.0) / t661;
            let t664 = t285 * t662 * t294;
            let t665 = t628 * t234;
            let t666 = t7 * t112;
            let t667 = t267 * t666;
            let t668 = t665 * t667;
            let t671 = t303 * t257;
            let t672 = t94 * t671;
            let t676 = t311 * t243;
            let t677 = t676 * t268;
            let t679 = t115 * t645;
            let t681 = -f64x8::splat(0.019190897446562643) * t585 * t315 - f64x8::splat(0.0041123351671205665) * t591 * t658 + f64x8::splat(0.008224670334241133) * t664 * t668 + f64x8::splat(0.008224670334241133) * t287 * t672 + t224 * t318 + t266 * t677 + t106 * t679;
            let t682 = t263 * t681;
            let t684 = -f64x8::splat(0.019190897446562643) * t585 * t290 - f64x8::splat(0.0041123351671205665) * t591 * t592 + f64x8::splat(0.008224670334241133) * t287 * t597 - f64x8::splat(0.008224670334241133) * t287 * t603 + t606 * t140 + t646 * t140 - t648 * t274 - t454 * t321 - t520 * t321 + f64x8::splat(2.0) * t260 * t653 - t260 * t682;
            let t688 = t154 * t77;
            let t689 = t567 * t276;
            let t690 = t574 * t689;
            let tv2rhosigma0 = t573 * t575 + f64x8::splat(0.03377372788077926) * t578 * t581 + t283 * t284 * t684 * t278 - t688 * t690;
            acc_v2rhosigma = tv2rhosigma0;
            let t693 = t311 * t105 * t139;
            let t694 = t94 * t693;
            let t697 = t601 * t320;
            let t698 = t600 * t697;
            let t701 = t303 * v_sigma;
            let t704 = t115 * t121;
            let t707 = t122 * t125 * t129;
            let t710 = f64x8::splat(0.0008074551218828078) * t302 * t701 + t106 * t704 * t707 / f64x8::splat(1536.0);
            let t711 = t98 * t710;
            let t715 = t320 * t320;
            let t716 = t525 * t715;
            let t719 = t303 * t311;
            let t720 = t94 * t719;
            let t723 = t115 * t710;
            let t725 = f64x8::splat(0.016449340668482266) * t287 * t720 + t106 * t723;
            let t726 = t263 * t725;
            let t728 = f64x8::splat(0.016449340668482266) * t287 * t694 - f64x8::splat(0.016449340668482266) * t287 * t698 + t711 * t140 - f64x8::splat(2.0) * t648 * t321 + f64x8::splat(2.0) * t260 * t716 - t260 * t726;
            let t732 = t323 * t323;
            let tv2sigma20 = t283 * t284 * t728 * t278 - t283 * t284 * t732 * t567;
            acc_v2sigma2 = tv2sigma20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
