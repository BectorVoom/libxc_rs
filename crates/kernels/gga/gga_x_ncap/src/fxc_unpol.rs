//! GGA_X_NCAP fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ncap.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_ncap_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_alpha: f64,
    param_beta: f64,
    param_mu: f64,
    param_zeta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5::<f64>(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3::<f64>(zeta_threshold);
        let t15 = pow_1_3::<f64>(t11);
        let t17 = piecewise3::<f64>(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3::<f64>(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = t20 * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3::<f64>(t22);
        let t24 = 1.0 / t23;
        let t25 = t21 * t24;
        let t26 = f64::sqrt(sigma[ip]);
        let t27 = M_CBRT2;
        let t28 = t26 * t27;
        let t30 = 1.0 / t18 / rho[ip];
        let t31 = t28 * t30;
        let t33 = t25 * t31 / 12.0;
        let t34 = f64::tanh(t33);
        let t35 = param_mu * t34;
        let t36 = f64::ln(t33 + f64::sqrt(t33 * t33 + 1.0));
        let t37 = 1.0 - param_zeta;
        let t39 = t37 * t21 * t24;
        let t40 = 1.0 + t33;
        let t41 = f64::ln(t40);
        let t42 = t30 * t41;
        let t46 = param_zeta * t21 * t24;
        let t51 = 1.0 + param_alpha * (t39 * t28 * t42 / 12.0 + t46 * t31 / 12.0);
        let t52 = t36 * t51;
        let t53 = param_beta * t34;
        let t55 = t53 * t36 + 1.0;
        let t56 = 1.0 / t55;
        let t57 = t52 * t56;
        let t59 = t35 * t57 + 1.0;
        let t63 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t59);
        let tzk0 = 2.0 * t63;
        zk[ip] += tzk0;
        let t64 = t18 * t18;
        let t66 = t17 / t64;
        let t70 = param_mu * t21;
        let t71 = t24 * t26;
        let t72 = t71 * t27;
        let t73 = t70 * t72;
        let t74 = rho[ip] * rho[ip];
        let t76 = 1.0 / t18 / t74;
        let t77 = t34 * t34;
        let t78 = 1.0 - t77;
        let t79 = t76 * t78;
        let t80 = t79 * t57;
        let t84 = t35 * t25 * t26;
        let t85 = t27 * t76;
        let t86 = t23 * t23;
        let t87 = 1.0 / t86;
        let t88 = t20 * t87;
        let t89 = t27 * t27;
        let t90 = sigma[ip] * t89;
        let t92 = 1.0 / t64 / t74;
        let t96 = 6.0 * t88 * t90 * t92 + 144.0;
        let t97 = f64::sqrt(t96);
        let t98 = 1.0 / t97;
        let t100 = t98 * t51 * t56;
        let t101 = t85 * t100;
        let t104 = t35 * t36;
        let t105 = t76 * t41;
        let t110 = t37 * t20 * t87;
        let t111 = t74 * rho[ip];
        let t113 = 1.0 / t64 / t111;
        let t114 = 1.0 / t40;
        let t115 = t113 * t114;
        let t119 = t28 * t76;
        let t122 = -t39 * t28 * t105 / 9.0 - t110 * t90 * t115 / 18.0 - t46 * t119 / 9.0;
        let t123 = param_alpha * t122;
        let t124 = t123 * t56;
        let t126 = t55 * t55;
        let t127 = 1.0 / t126;
        let t128 = t51 * t127;
        let t129 = param_beta * t21;
        let t130 = t129 * t71;
        let t131 = t78 * t36;
        let t132 = t85 * t131;
        let t135 = t53 * t25;
        let t136 = t76 * t98;
        let t140 = -t130 * t132 / 9.0 - 4.0 / 3.0 * t135 * t28 * t136;
        let t141 = t128 * t140;
        let t143 = -t73 * t80 / 9.0 - 4.0 / 3.0 * t84 * t101 + t104 * t124 - t104 * t141;
        let t148 = piecewise3::<f64>(t2, 0.0, -t6 * t66 * t59 / 8.0 - 3.0 / 8.0 * t6 * t19 * t143);
        let tvrho0 = 2.0 * rho[ip] * t148 + 2.0 * t63;
        vrho[ip] += tvrho0;
        let t151 = 1.0 / t26;
        let t152 = t24 * t151;
        let t153 = t152 * t27;
        let t154 = t70 * t153;
        let t155 = t30 * t78;
        let t156 = t155 * t57;
        let t160 = t35 * t25 * t151;
        let t161 = t27 * t30;
        let t162 = t161 * t100;
        let t165 = t151 * t27;
        let t169 = t89 * t92;
        let t173 = t165 * t30;
        let t176 = t39 * t165 * t42 / 24.0 + t110 * t169 * t114 / 48.0 + t46 * t173 / 24.0;
        let t177 = param_alpha * t176;
        let t178 = t177 * t56;
        let t180 = t129 * t152;
        let t181 = t161 * t131;
        let t184 = t30 * t98;
        let t188 = t180 * t181 / 24.0 + t135 * t165 * t184 / 2.0;
        let t189 = t128 * t188;
        let t191 = t154 * t156 / 24.0 + t160 * t162 / 2.0 + t104 * t178 - t104 * t189;
        let t195 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t191);
        let tvsigma0 = 2.0 * rho[ip] * t195;
        vsigma[ip] += tvsigma0;
        let t200 = t17 / t64 / rho[ip];
        let t208 = 1.0 / t18 / t111;
        let t209 = t208 * t78;
        let t210 = t209 * t57;
        let t213 = param_mu * t20;
        let t214 = t87 * sigma[ip];
        let t216 = t213 * t214 * t89;
        let t217 = t74 * t74;
        let t219 = 1.0 / t64 / t217;
        let t220 = t219 * t34;
        let t222 = t220 * t78 * t57;
        let t225 = t219 * t78;
        let t226 = t225 * t100;
        let t229 = t79 * t36;
        let t230 = t229 * t124;
        let t233 = t229 * t141;
        let t236 = t27 * t208;
        let t237 = t236 * t100;
        let t240 = 1.0 / t22;
        let t241 = t26 * sigma[ip];
        let t242 = t240 * t241;
        let t243 = t35 * t242;
        let t244 = t217 * t74;
        let t245 = 1.0 / t244;
        let t247 = 1.0 / t97 / t96;
        let t248 = t245 * t247;
        let t249 = t51 * t56;
        let t250 = t248 * t249;
        let t253 = t85 * t98;
        let t254 = t253 * t124;
        let t257 = t253 * t141;
        let t260 = t208 * t41;
        let t264 = t219 * t114;
        let t268 = t37 * t240;
        let t269 = t241 * t245;
        let t270 = t40 * t40;
        let t271 = 1.0 / t270;
        let t275 = t28 * t208;
        let t278 = 7.0 / 27.0 * t39 * t28 * t260 + 5.0 / 18.0 * t110 * t90 * t264 - 2.0 / 27.0 * t268 * t269 * t271 + 7.0 / 27.0 * t46 * t275;
        let t279 = param_alpha * t278;
        let t280 = t279 * t56;
        let t282 = t127 * t140;
        let t283 = t123 * t282;
        let t287 = 1.0 / t126 / t55;
        let t288 = t51 * t287;
        let t289 = t140 * t140;
        let t290 = t288 * t289;
        let t293 = t236 * t131;
        let t296 = param_beta * t20;
        let t297 = t296 * t214;
        let t298 = t89 * t219;
        let t299 = t34 * t78;
        let t300 = t299 * t36;
        let t301 = t298 * t300;
        let t304 = t78 * t98;
        let t305 = t298 * t304;
        let t308 = t208 * t98;
        let t312 = t53 * t240;
        let t316 = 7.0 / 27.0 * t130 * t293 - 4.0 / 27.0 * t297 * t301 + 16.0 / 9.0 * t297 * t305 + 28.0 / 9.0 * t135 * t28 * t308 - 128.0 * t312 * t269 * t247;
        let t317 = t128 * t316;
        let t319 = 7.0 / 27.0 * t73 * t210 - 4.0 / 27.0 * t216 * t222 + 16.0 / 9.0 * t216 * t226 - 2.0 / 9.0 * t73 * t230 + 2.0 / 9.0 * t73 * t233 + 28.0 / 9.0 * t84 * t237 - 128.0 * t243 * t250 - 8.0 / 3.0 * t84 * t254 + 8.0 / 3.0 * t84 * t257 + t104 * t280 - 2.0 * t104 * t283 + 2.0 * t104 * t290 - t104 * t317;
        let t324 = piecewise3::<f64>(t2, 0.0, t6 * t200 * t59 / 12.0 - t6 * t66 * t143 / 4.0 - 3.0 / 8.0 * t6 * t19 * t319);
        let tv2rho20 = 2.0 * rho[ip] * t324 + 4.0 * t148;
        v2rho2[ip] += tv2rho20;
        let t332 = t87 * t89;
        let t334 = t213 * t332 * t113;
        let t335 = t299 * t57;
        let t338 = t213 * t332;
        let t339 = t113 * t78;
        let t340 = t339 * t100;
        let t343 = t155 * t36;
        let t344 = t343 * t124;
        let t347 = t343 * t141;
        let t352 = t240 * t26;
        let t353 = t35 * t352;
        let t354 = t217 * rho[ip];
        let t355 = 1.0 / t354;
        let t356 = t355 * t247;
        let t357 = t356 * t249;
        let t360 = t161 * t98;
        let t361 = t360 * t124;
        let t364 = t360 * t141;
        let t367 = t229 * t178;
        let t370 = t253 * t178;
        let t376 = t89 * t113;
        let t384 = t165 * t76;
        let t387 = -t39 * t165 * t105 / 18.0 - t110 * t376 * t114 / 12.0 + t268 * t355 * t271 * t26 / 36.0 - t46 * t384 / 18.0;
        let t388 = param_alpha * t387;
        let t389 = t388 * t56;
        let t391 = t177 * t282;
        let t393 = t229 * t189;
        let t396 = t253 * t189;
        let t399 = t127 * t188;
        let t400 = t123 * t399;
        let t402 = t188 * t140;
        let t403 = t288 * t402;
        let t408 = t296 * t332;
        let t409 = t113 * t34;
        let t413 = t296 * t87;
        let t414 = t376 * t304;
        let t424 = -t180 * t132 / 18.0 + t408 * t409 * t131 / 18.0 - 2.0 / 3.0 * t413 * t414 - 2.0 / 3.0 * t135 * t165 * t136 + 48.0 * t312 * t26 * t355 * t247;
        let t425 = t128 * t424;
        let t427 = -t154 * t80 / 18.0 + t334 * t335 / 18.0 - 2.0 / 3.0 * t338 * t340 + t154 * t344 / 24.0 - t154 * t347 / 24.0 - 2.0 / 3.0 * t160 * t101 + 48.0 * t353 * t357 + t160 * t361 / 2.0 - t160 * t364 / 2.0 - t73 * t367 / 9.0 - 4.0 / 3.0 * t84 * t370 + t104 * t389 - t104 * t391 + t73 * t393 / 9.0 + 4.0 / 3.0 * t84 * t396 - t104 * t400 + 2.0 * t104 * t403 - t104 * t425;
        let t432 = piecewise3::<f64>(t2, 0.0, -t6 * t66 * t191 / 8.0 - 3.0 / 8.0 * t6 * t19 * t427);
        let tv2rhosigma0 = 2.0 * rho[ip] * t432 + 2.0 * t195;
        v2rhosigma[ip] += tv2rhosigma0;
        let t435 = 1.0 / t241;
        let t436 = t24 * t435;
        let t437 = t436 * t27;
        let t438 = t70 * t437;
        let t441 = 1.0 / sigma[ip];
        let t442 = t87 * t441;
        let t444 = t213 * t442 * t89;
        let t447 = t92 * t34 * t78 * t57;
        let t450 = t92 * t78;
        let t451 = t450 * t100;
        let t454 = t343 * t178;
        let t457 = t343 * t189;
        let t461 = t35 * t25 * t435;
        let t464 = t240 * t151;
        let t465 = t35 * t464;
        let t466 = 1.0 / t217;
        let t467 = t466 * t247;
        let t468 = t467 * t249;
        let t471 = t360 * t178;
        let t473 = t360 * t189;
        let t475 = t435 * t27;
        let t479 = t441 * t89;
        let t480 = t92 * t114;
        let t488 = t475 * t30;
        let t491 = -t39 * t475 * t42 / 48.0 + t110 * t479 * t480 / 96.0 - t268 * t466 * t271 * t151 / 96.0 - t46 * t488 / 48.0;
        let t492 = param_alpha * t491;
        let t493 = t492 * t56;
        let t495 = t177 * t399;
        let t498 = t188 * t188;
        let t499 = t288 * t498;
        let t502 = t129 * t436;
        let t505 = t296 * t442;
        let t506 = t169 * t300;
        let t509 = t169 * t304;
        let t519 = -t502 * t181 / 48.0 - t505 * t506 / 48.0 + t505 * t509 / 4.0 - t135 * t475 * t184 / 4.0 - 18.0 * t312 * t151 * t466 * t247;
        let t520 = t128 * t519;
        let t522 = -t438 * t156 / 48.0 - t444 * t447 / 48.0 + t444 * t451 / 4.0 + t154 * t454 / 12.0 - t154 * t457 / 12.0 - t461 * t162 / 4.0 - 18.0 * t465 * t468 + t160 * t471 - t160 * t473 + t104 * t493 - 2.0 * t104 * t495 + 2.0 * t104 * t499 - t104 * t520;
        let t526 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t522);
        let tv2sigma20 = 2.0 * rho[ip] * t526;
        v2sigma2[ip] += tv2sigma20;
    }
}
