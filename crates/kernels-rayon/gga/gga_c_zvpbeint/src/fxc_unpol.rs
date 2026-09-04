//! GGA_C_ZVPBEINT fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_zvpbeint.c`
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
pub fn gga_c_zvpbeint_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_alpha: f64,
    param_omega: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alpha = f64x8::splat(param_alpha);
    let param_omega = f64x8::splat(param_omega);
    let param_beta = f64x8::splat(param_beta);
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
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t10 = t4 * t6 / t7;
            let t12 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t10;
            let t13 = ((t10).sqrt());
            let t16 = ((t10) * (t10).sqrt());
            let t18 = t1 * t1;
            let t19 = t3 * t3;
            let t20 = t18 * t19;
            let t21 = t7 * t7;
            let t24 = t20 * t5 / t21;
            let t26 = f64x8::splat(3.79785) * t13 + f64x8::splat(0.8969) * t10 + f64x8::splat(0.204775) * t16 + f64x8::splat(0.123235) * t24;
            let t29 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t26;
            let t30 = (simd::ln(t29));
            let t32 = f64x8::splat(0.0621814) * t12 * t30;
            let t33 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = ((t33).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t10;
            let t50 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t10 + f64x8::splat(0.1100325) * t16 + f64x8::splat(0.1241775) * t24;
            let t53 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t50;
            let t54 = (simd::ln(t53));
            let t57 = f64x8::splat(0.0197516734986138) * t43 * t45 * t54;
            let t58 = ((v_sigma).sqrt());
            let t59 = t58 * v_sigma;
            let t60 = param_alpha * t59;
            let t61 = v_rho * v_rho;
            let t62 = t61 * t61;
            let t63 = f64x8::splat(1.0) / t62;
            let t66 = f64x8::splat(1.0) / t13 / t10;
            let t67 = f64x8::splat(1.0) / t3;
            let t68 = t18 * t67;
            let t70 = t68 * t5 * t7;
            let t71 = ((t70).sqrt());
            let t72 = t66 * t71;
            let t74 = (((f64x8::splat(1e-20)).simd_lt(f64x8::splat(0.0))).select(f64x8::splat(0.0), f64x8::splat(1e-20)));
            let t76 = (simd::pow(t74, param_omega / f64x8::splat(2.0)));
            let t77 = t72 * t76;
            let t80 = (simd::exp(-t60 * t63 * t77 / f64x8::splat(16.0)));
            let t81 = (simd::ln(f64x8::splat(2.0)));
            let t82 = f64x8::splat(1.0) - t81;
            let t83 = t80 * t82;
            let t84 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t85 = f64x8::splat(1.0) / t84;
            let t86 = t34 * t34;
            let t87 = ((t33).select(t86, f64x8::splat(1.0)));
            let t88 = t87 * t87;
            let t89 = t88 * t87;
            let t90 = t85 * t89;
            let t92 = f64x8::splat(1.0) / t7 / t61;
            let t95 = f64x8::splat(1.0) / t88;
            let t97 = t67 * t5;
            let t98 = t95 * t18 * t97;
            let t101 = f64x8::splat(1.0) / t82;
            let t102 = param_beta * t101;
            let t105 = f64x8::splat(1.0) / t89;
            let t108 = (simd::exp(-(-t32 + t57) * t101 * t84 * t105));
            let t109 = t108 - f64x8::splat(1.0);
            let t110 = f64x8::splat(1.0) / t109;
            let t111 = t84 * t110;
            let t112 = v_sigma * v_sigma;
            let t114 = t102 * t111 * t112;
            let t116 = f64x8::splat(1.0) / t21 / t62;
            let t117 = t39 * t39;
            let t118 = t116 * t117;
            let t119 = t88 * t88;
            let t120 = f64x8::splat(1.0) / t119;
            let t121 = t118 * t120;
            let t122 = f64x8::splat(1.0) / t19;
            let t123 = t1 * t122;
            let t124 = t123 * t6;
            let t125 = t121 * t124;
            let t128 = v_sigma * t92 * t39 * t98 / f64x8::splat(96.0) + t114 * t125 / f64x8::splat(3072.0);
            let t129 = param_beta * t128;
            let t133 = t102 * t111 * t128 + f64x8::splat(1.0);
            let t134 = f64x8::splat(1.0) / t133;
            let t135 = t101 * t84 * t134;
            let t137 = t129 * t135 + f64x8::splat(1.0);
            let t138 = (simd::ln(t137));
            let t139 = t90 * t138;
            let t140 = t83 * t139;
            let tzk0 = -t32 + t57 + t140;
            acc_zk = tzk0;
            let t142 = f64x8::splat(1.0) / t7 / v_rho;
            let t143 = t6 * t142;
            let t145 = t4 * t143 * t30;
            let t146 = f64x8::splat(0.0011073470983333333) * t145;
            let t147 = t26 * t26;
            let t148 = f64x8::splat(1.0) / t147;
            let t149 = t12 * t148;
            let t151 = f64x8::splat(1.0) / t13 * t1;
            let t152 = t3 * t6;
            let t153 = t152 * t142;
            let t154 = t151 * t153;
            let t156 = t4 * t143;
            let t158 = ((t10).sqrt());
            let t159 = t158 * t1;
            let t160 = t159 * t153;
            let t164 = t5 / t21 / v_rho;
            let t165 = t20 * t164;
            let t167 = -f64x8::splat(0.632975) * t154 - f64x8::splat(0.29896666666666666) * t156 - f64x8::splat(0.1023875) * t160 - f64x8::splat(0.08215666666666667) * t165;
            let t168 = f64x8::splat(1.0) / t29;
            let t169 = t167 * t168;
            let t170 = t149 * t169;
            let t171 = f64x8::splat(1.0) * t170;
            let t172 = t43 * t1;
            let t175 = t172 * t152 * t142 * t54;
            let t176 = f64x8::splat(0.00018311447306006544) * t175;
            let t177 = t43 * t45;
            let t178 = t50 * t50;
            let t179 = f64x8::splat(1.0) / t178;
            let t184 = -f64x8::splat(0.8630833333333333) * t154 - f64x8::splat(0.301925) * t156 - f64x8::splat(0.05501625) * t160 - f64x8::splat(0.082785) * t165;
            let t186 = f64x8::splat(1.0) / t53;
            let t187 = t179 * t184 * t186;
            let t188 = t177 * t187;
            let t189 = f64x8::splat(0.5848223622634646) * t188;
            let t190 = t62 * v_rho;
            let t191 = f64x8::splat(1.0) / t190;
            let t196 = f64x8::splat(1.0) / t7 / t190;
            let t199 = f64x8::splat(1.0) / t13 / t24 / f64x8::splat(4.0);
            let t200 = t196 * t199;
            let t202 = t71 * t76;
            let t203 = t4 * t6;
            let t204 = t202 * t203;
            let t207 = t116 * t66;
            let t209 = f64x8::splat(1.0) / t71;
            let t210 = t209 * t76;
            let t211 = t68 * t5;
            let t212 = t210 * t211;
            let t215 = t60 * t191 * t77 / f64x8::splat(4.0) - t60 * t200 * t204 / f64x8::splat(32.0) - t60 * t207 * t212 / f64x8::splat(96.0);
            let t216 = t215 * t80;
            let t217 = t216 * t82;
            let t218 = t217 * t139;
            let t219 = t83 * t85;
            let t220 = t61 * v_rho;
            let t222 = f64x8::splat(1.0) / t7 / t220;
            let t227 = t82 * t82;
            let t228 = f64x8::splat(1.0) / t227;
            let t229 = param_beta * t228;
            let t230 = t84 * t84;
            let t231 = t229 * t230;
            let t232 = t109 * t109;
            let t233 = f64x8::splat(1.0) / t232;
            let t234 = t233 * t112;
            let t235 = t234 * t116;
            let t236 = t231 * t235;
            let t238 = f64x8::splat(1.0) / t119 / t89;
            let t239 = t117 * t238;
            let t240 = t239 * t1;
            let t241 = t122 * t6;
            let t242 = t146 + t171 - t176 - t189;
            let t243 = t242 * t108;
            let t244 = t241 * t243;
            let t245 = t240 * t244;
            let t249 = f64x8::splat(1.0) / t21 / t190;
            let t250 = t249 * t117;
            let t251 = t250 * t120;
            let t252 = t251 * t124;
            let t255 = -f64x8::splat(7.0) / f64x8::splat(288.0) * v_sigma * t222 * t39 * t98 + t236 * t245 / f64x8::splat(3072.0) - f64x8::splat(7.0) / f64x8::splat(4608.0) * t114 * t252;
            let t256 = param_beta * t255;
            let t258 = t129 * t101;
            let t259 = t133 * t133;
            let t260 = f64x8::splat(1.0) / t259;
            let t261 = t84 * t260;
            let t263 = t229 * t230 * t233;
            let t264 = t128 * t242;
            let t265 = t105 * t108;
            let t270 = t102 * t111 * t255 + t263 * t264 * t265;
            let t271 = t261 * t270;
            let t273 = t256 * t135 - t258 * t271;
            let t274 = t89 * t273;
            let t275 = f64x8::splat(1.0) / t137;
            let t276 = t274 * t275;
            let t277 = t219 * t276;
            let tvrho0 = -t32 + t57 + t140 + v_rho * (t146 + t171 - t176 - t189 + t218 + t277);
            acc_vrho = tvrho0;
            let t280 = param_alpha * t58;
            let t282 = t63 * t66 * t71;
            let t284 = t76 * t80;
            let t285 = t284 * t82;
            let t286 = t285 * t139;
            let t288 = f64x8::splat(3.0) / f64x8::splat(32.0) * t280 * t282 * t286;
            let t294 = t102 * t111 * v_sigma;
            let t297 = t92 * t39 * t95 * t211 / f64x8::splat(96.0) + t294 * t125 / f64x8::splat(1536.0);
            let t298 = param_beta * t297;
            let t300 = param_beta * param_beta;
            let t301 = t300 * t128;
            let t302 = t301 * t228;
            let t303 = t230 * t260;
            let t305 = t303 * t110 * t297;
            let t307 = t298 * t135 - t302 * t305;
            let t308 = t89 * t307;
            let t309 = t308 * t275;
            let t310 = t219 * t309;
            let tvsigma0 = v_rho * (-t288 + t310);
            acc_vsigma = tvsigma0;
            let t318 = t6 * t92;
            let t320 = t4 * t318 * t30;
            let t321 = f64x8::splat(0.0014764627977777779) * t320;
            let t322 = t142 * t148;
            let t324 = t203 * t322 * t169;
            let t325 = f64x8::splat(0.035616666666666665) * t324;
            let t326 = t147 * t26;
            let t327 = f64x8::splat(1.0) / t326;
            let t328 = t12 * t327;
            let t329 = t167 * t167;
            let t330 = t329 * t168;
            let t331 = t328 * t330;
            let t332 = f64x8::splat(2.0) * t331;
            let t333 = t66 * t18;
            let t334 = t19 * t5;
            let t336 = f64x8::splat(1.0) / t21 / t61;
            let t337 = t334 * t336;
            let t338 = t333 * t337;
            let t340 = t152 * t92;
            let t341 = t151 * t340;
            let t343 = t4 * t318;
            let t345 = f64x8::splat(1.0)/((t10).sqrt());
            let t346 = t345 * t18;
            let t347 = t346 * t337;
            let t349 = t159 * t340;
            let t352 = t20 * t5 * t336;
            let t354 = -f64x8::splat(0.4219833333333333) * t338 + f64x8::splat(0.8439666666666666) * t341 + f64x8::splat(0.3986222222222222) * t343 + f64x8::splat(0.06825833333333334) * t347 + f64x8::splat(0.13651666666666668) * t349 + f64x8::splat(0.1369277777777778) * t352;
            let t355 = t354 * t168;
            let t356 = t149 * t355;
            let t357 = f64x8::splat(1.0) * t356;
            let t358 = t147 * t147;
            let t359 = f64x8::splat(1.0) / t358;
            let t360 = t12 * t359;
            let t361 = t29 * t29;
            let t362 = f64x8::splat(1.0) / t361;
            let t363 = t329 * t362;
            let t364 = t360 * t363;
            let t365 = f64x8::splat(16.081979498692537) * t364;
            let t368 = t172 * t152 * t92 * t54;
            let t369 = f64x8::splat(0.00024415263074675396) * t368;
            let t370 = t43 * t4;
            let t372 = t370 * t143 * t187;
            let t373 = f64x8::splat(0.01084358130030174) * t372;
            let t374 = t178 * t50;
            let t375 = f64x8::splat(1.0) / t374;
            let t376 = t184 * t184;
            let t378 = t375 * t376 * t186;
            let t379 = t177 * t378;
            let t380 = f64x8::splat(1.1696447245269292) * t379;
            let t387 = -f64x8::splat(0.5753888888888888) * t338 + f64x8::splat(1.1507777777777777) * t341 + f64x8::splat(0.4025666666666667) * t343 + f64x8::splat(0.0366775) * t347 + f64x8::splat(0.073355) * t349 + f64x8::splat(0.137975) * t352;
            let t389 = t179 * t387 * t186;
            let t390 = t177 * t389;
            let t391 = f64x8::splat(0.5848223622634646) * t390;
            let t392 = t178 * t178;
            let t393 = f64x8::splat(1.0) / t392;
            let t394 = t393 * t376;
            let t395 = t53 * t53;
            let t396 = f64x8::splat(1.0) / t395;
            let t397 = t394 * t396;
            let t398 = t177 * t397;
            let t399 = f64x8::splat(17.315859105681465) * t398;
            let t400 = t62 * t61;
            let t401 = f64x8::splat(1.0) / t400;
            let t402 = t60 * t401;
            let t406 = f64x8::splat(1.0) / t7 / t400;
            let t416 = f64x8::splat(1.0) / t21 / t400;
            let t421 = f64x8::splat(1.0) / t13 / t2 * v_rho / f64x8::splat(48.0);
            let t424 = t20 * t5;
            let t425 = t202 * t424;
            let t429 = t199 * t209 * t76;
            let t435 = f64x8::splat(1.0) / t71 / t70;
            let t436 = t435 * t76;
            let t437 = t436 * t124;
            let t440 = -f64x8::splat(5.0) / f64x8::splat(4.0) * t402 * t77 + f64x8::splat(7.0) / f64x8::splat(24.0) * t60 * t406 * t199 * t204 + f64x8::splat(13.0) / f64x8::splat(144.0) * t60 * t249 * t66 * t212 - f64x8::splat(5.0) / f64x8::splat(48.0) * t60 * t416 * t421 * t425 - t402 * t429 / f64x8::splat(8.0) + t60 * t196 * t66 * t437 / f64x8::splat(192.0);
            let t441 = t440 * t80;
            let t442 = t441 * t82;
            let t443 = t442 * t139;
            let t444 = t215 * t215;
            let t445 = t444 * t80;
            let t446 = t445 * t82;
            let t447 = t446 * t139;
            let t448 = t273 * t275;
            let t449 = t90 * t448;
            let t450 = t217 * t449;
            let t453 = f64x8::splat(1.0) / t7 / t62;
            let t459 = f64x8::splat(1.0) / t227 / t82;
            let t460 = param_beta * t459;
            let t461 = t230 * t84;
            let t462 = t460 * t461;
            let t464 = f64x8::splat(1.0) / t232 / t109;
            let t465 = t464 * t112;
            let t466 = t465 * t116;
            let t467 = t462 * t466;
            let t468 = t119 * t119;
            let t470 = f64x8::splat(1.0) / t468 / t88;
            let t472 = t117 * t470 * t1;
            let t473 = t242 * t242;
            let t474 = t108 * t108;
            let t475 = t473 * t474;
            let t477 = t472 * t241 * t475;
            let t480 = t234 * t249;
            let t481 = t231 * t480;
            let t484 = -t321 - t325 - t332 + t357 + t365 + t369 + t373 + t380 - t391 - t399;
            let t485 = t484 * t108;
            let t487 = t240 * t241 * t485;
            let t490 = t462 * t235;
            let t491 = t473 * t108;
            let t493 = t472 * t241 * t491;
            let t498 = t416 * t117 * t120 * t124;
            let t501 = f64x8::splat(35.0) / f64x8::splat(432.0) * v_sigma * t453 * t39 * t98 + t467 * t477 / f64x8::splat(1536.0) - f64x8::splat(7.0) / f64x8::splat(2304.0) * t481 * t245 + t236 * t487 / f64x8::splat(3072.0) - t490 * t493 / f64x8::splat(3072.0) + f64x8::splat(119.0) / f64x8::splat(13824.0) * t114 * t498;
            let t502 = param_beta * t501;
            let t504 = t256 * t101;
            let t508 = f64x8::splat(1.0) / t259 / t133;
            let t509 = t84 * t508;
            let t510 = t270 * t270;
            let t511 = t509 * t510;
            let t515 = t460 * t461 * t464;
            let t516 = t128 * t473;
            let t518 = f64x8::splat(1.0) / t119 / t88;
            let t519 = t518 * t474;
            let t523 = t255 * t242;
            let t527 = t128 * t484;
            let t531 = t460 * t461 * t233;
            let t532 = t518 * t108;
            let t537 = t102 * t111 * t501 + f64x8::splat(2.0) * t263 * t523 * t265 + t263 * t527 * t265 + f64x8::splat(2.0) * t515 * t516 * t519 - t531 * t516 * t532;
            let t538 = t261 * t537;
            let t540 = t502 * t135 + f64x8::splat(2.0) * t258 * t511 - t258 * t538 - f64x8::splat(2.0) * t504 * t271;
            let t541 = t89 * t540;
            let t543 = t219 * t541 * t275;
            let t544 = t273 * t273;
            let t545 = t89 * t544;
            let t546 = t137 * t137;
            let t547 = f64x8::splat(1.0) / t546;
            let t549 = t219 * t545 * t547;
            let t550 = -t321 - t325 - t332 + t357 + t365 + t369 + t373 + t380 - t391 - t399 + t443 + t447 + f64x8::splat(2.0) * t450 + t543 - t549;
            let tv2rho20 = f64x8::splat(0.0022146941966666666) * t145 + f64x8::splat(2.0) * t170 - f64x8::splat(0.0003662289461201309) * t175 - f64x8::splat(1.1696447245269292) * t188 + f64x8::splat(2.0) * t218 + f64x8::splat(2.0) * t277 + v_rho * t550;
            acc_v2rho2 = tv2rho20;
            let t553 = t191 * t66 * t71;
            let t555 = t280 * t553 * t286;
            let t557 = t280 * t196;
            let t558 = t199 * t71;
            let t559 = t558 * t284;
            let t560 = t557 * t559;
            let t561 = t82 * t85;
            let t562 = t561 * t89;
            let t563 = t138 * t1;
            let t564 = t563 * t152;
            let t565 = t562 * t564;
            let t566 = t560 * t565;
            let t568 = t280 * t116;
            let t569 = t66 * t209;
            let t570 = t569 * t284;
            let t571 = t568 * t570;
            let t572 = t138 * t18;
            let t573 = t572 * t97;
            let t574 = t562 * t573;
            let t575 = t571 * t574;
            let t577 = t280 * t63;
            let t578 = t577 * t77;
            let t579 = t578 * t218;
            let t581 = t578 * t277;
            let t583 = t307 * t275;
            let t584 = t90 * t583;
            let t585 = t217 * t584;
            let t590 = t233 * v_sigma;
            let t591 = t590 * t116;
            let t592 = t231 * t591;
            let t597 = -f64x8::splat(7.0) / f64x8::splat(288.0) * t222 * t39 * t95 * t211 + t592 * t245 / f64x8::splat(1536.0) - f64x8::splat(7.0) / f64x8::splat(2304.0) * t294 * t252;
            let t598 = param_beta * t597;
            let t600 = t298 * t101;
            let t602 = t300 * t255;
            let t603 = t602 * t228;
            let t605 = t228 * t230;
            let t606 = t301 * t605;
            let t607 = t508 * t110;
            let t608 = t297 * t270;
            let t609 = t607 * t608;
            let t612 = t459 * t461;
            let t613 = t612 * t260;
            let t614 = t301 * t613;
            let t615 = t233 * t297;
            let t617 = t242 * t105 * t108;
            let t618 = t615 * t617;
            let t621 = t303 * t110 * t597;
            let t623 = t598 * t135 - t600 * t271 - t302 * t621 - t603 * t305 + f64x8::splat(2.0) * t606 * t609 - t614 * t618;
            let t624 = t89 * t623;
            let t626 = t219 * t624 * t275;
            let t627 = t547 * t273;
            let t628 = t308 * t627;
            let t629 = t219 * t628;
            let tv2rhosigma0 = -t288 + t310 + v_rho * (f64x8::splat(3.0) / f64x8::splat(8.0) * t555 - f64x8::splat(3.0) / f64x8::splat(64.0) * t566 - t575 / f64x8::splat(64.0) - f64x8::splat(3.0) / f64x8::splat(32.0) * t579 - f64x8::splat(3.0) / f64x8::splat(32.0) * t581 + t585 + t626 - t629);
            acc_v2rhosigma = tv2rhosigma0;
            let t633 = param_alpha / t58;
            let t636 = f64x8::splat(3.0) / f64x8::splat(64.0) * t633 * t282 * t286;
            let t637 = param_alpha * param_alpha;
            let t638 = t637 * v_sigma;
            let t639 = t638 * t416;
            let t640 = t2 * t18;
            let t641 = t640 * t67;
            let t642 = t639 * t641;
            let t643 = t76 * t76;
            let t644 = t5 * t643;
            let t645 = t644 * t80;
            let t646 = t82 * t89;
            let t647 = t646 * t138;
            let t648 = t645 * t647;
            let t650 = f64x8::splat(3.0) / f64x8::splat(16384.0) * t642 * t648;
            let t652 = f64x8::splat(3.0) / f64x8::splat(16.0) * t578 * t310;
            let t653 = t300 * t228;
            let t654 = t230 * t110;
            let t660 = t117 * t120 * t1 * t241 * t134;
            let t663 = t297 * t297;
            let t664 = t300 * t663;
            let t665 = t664 * t228;
            let t666 = t303 * t110;
            let t669 = t300 * param_beta;
            let t670 = t669 * t128;
            let t671 = t670 * t459;
            let t672 = t461 * t508;
            let t674 = t672 * t233 * t663;
            let t678 = t461 * t260 * t233;
            let t679 = t671 * t678;
            let t682 = t653 * t654 * t116 * t660 / f64x8::splat(1536.0) - f64x8::splat(2.0) * t665 * t666 + f64x8::splat(2.0) * t671 * t674 - t679 * t125 / f64x8::splat(1536.0);
            let t683 = t89 * t682;
            let t685 = t219 * t683 * t275;
            let t686 = t307 * t307;
            let t687 = t89 * t686;
            let t689 = t219 * t687 * t547;
            let tv2sigma20 = v_rho * (-t636 + t650 - t652 + t685 - t689);
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
