//! GGA_K_APBEINT lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_apbeint.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_apbeint_lxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    param_alpha: f64,
    param_kappa: f64,
    param_muGE: f64,
    param_muPBE: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5::<f64>(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3::<f64>(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3::<f64>(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3::<f64>(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3::<f64>(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = param_muPBE - param_muGE;
        let t25 = t24 * param_alpha;
        let t26 = M_CBRT6;
        let t27 = M_PI * M_PI;
        let t28 = pow_1_3::<f64>(t27);
        let t29 = t28 * t28;
        let t30 = 1.0 / t29;
        let t31 = t26 * t30;
        let t32 = t25 * t31;
        let t33 = M_CBRT2;
        let t34 = t33 * t33;
        let t35 = sigma[ip] * t34;
        let t36 = rho[ip] * rho[ip];
        let t38 = 1.0 / t22 / t36;
        let t41 = t35 * t38;
        let t44 = 1.0 + param_alpha * t26 * t30 * t41 / 24.0;
        let t45 = 1.0 / t44;
        let t46 = t38 * t45;
        let t51 = (param_muGE + t32 * t35 * t46 / 24.0) * t26;
        let t52 = t51 * t30;
        let t55 = param_kappa + t52 * t41 / 24.0;
        let t60 = 1.0 + param_kappa * (1.0 - param_kappa / t55);
        let t64 = piecewise3::<f64>(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t60);
        let tzk0 = 2.0 * t64;
        zk[ip] += tzk0;
        let t65 = 1.0 / t21;
        let t66 = t20 * t65;
        let t70 = t7 * t20;
        let t71 = param_kappa * param_kappa;
        let t72 = t22 * t71;
        let t73 = t55 * t55;
        let t74 = 1.0 / t73;
        let t75 = t36 * rho[ip];
        let t77 = 1.0 / t22 / t75;
        let t78 = t77 * t45;
        let t82 = param_alpha * param_alpha;
        let t83 = t24 * t82;
        let t84 = t26 * t26;
        let t86 = 1.0 / t28 / t27;
        let t87 = t84 * t86;
        let t88 = t83 * t87;
        let t89 = sigma[ip] * sigma[ip];
        let t90 = t89 * t33;
        let t91 = t36 * t36;
        let t92 = t91 * t36;
        let t94 = 1.0 / t21 / t92;
        let t95 = t44 * t44;
        let t96 = 1.0 / t95;
        let t97 = t94 * t96;
        let t102 = (-t32 * t35 * t78 / 9.0 + t88 * t90 * t97 / 108.0) * t26;
        let t103 = t102 * t30;
        let t106 = t35 * t77;
        let t109 = t103 * t41 / 24.0 - t52 * t106 / 9.0;
        let t110 = t74 * t109;
        let t115 = piecewise3::<f64>(t2, 0.0, t7 * t66 * t60 / 10.0 + 3.0 / 20.0 * t70 * t72 * t110);
        let tvrho0 = 2.0 * rho[ip] * t115 + 2.0 * t64;
        vrho[ip] += tvrho0;
        let t118 = t25 * t26;
        let t119 = t30 * t34;
        let t124 = t91 * rho[ip];
        let t127 = 1.0 / t21 / t124 * t96;
        let t132 = (t118 * t119 * t46 / 24.0 - t88 * sigma[ip] * t33 * t127 / 288.0) * t26;
        let t133 = t132 * t30;
        let t135 = t119 * t38;
        let t138 = t133 * t41 / 24.0 + t51 * t135 / 24.0;
        let t139 = t74 * t138;
        let t143 = piecewise3::<f64>(t2, 0.0, 3.0 / 20.0 * t70 * t72 * t139);
        let tvsigma0 = 2.0 * rho[ip] * t143;
        vsigma[ip] += tvsigma0;
        let t147 = 1.0 / t21 / rho[ip];
        let t148 = t20 * t147;
        let t152 = t65 * t71;
        let t157 = 1.0 / t73 / t55;
        let t158 = t109 * t109;
        let t159 = t157 * t158;
        let t164 = 1.0 / t22 / t91;
        let t165 = t164 * t45;
        let t169 = t91 * t75;
        let t171 = 1.0 / t21 / t169;
        let t172 = t171 * t96;
        let t177 = t24 * t82 * param_alpha;
        let t178 = t27 * t27;
        let t179 = 1.0 / t178;
        let t180 = t177 * t179;
        let t181 = t89 * sigma[ip];
        let t182 = t91 * t91;
        let t183 = t182 * t36;
        let t184 = 1.0 / t183;
        let t187 = 1.0 / t95 / t44;
        let t192 = (11.0 / 27.0 * t32 * t35 * t165 - t88 * t90 * t172 / 12.0 + 2.0 / 81.0 * t180 * t181 * t184 * t187) * t26;
        let t193 = t192 * t30;
        let t198 = t35 * t164;
        let t201 = t193 * t41 / 24.0 - 2.0 / 9.0 * t103 * t106 + 11.0 / 27.0 * t52 * t198;
        let t202 = t74 * t201;
        let t207 = piecewise3::<f64>(t2, 0.0, -t7 * t148 * t60 / 30.0 + t70 * t152 * t110 / 5.0 - 3.0 / 10.0 * t70 * t72 * t159 + 3.0 / 20.0 * t70 * t72 * t202);
        let tv2rho20 = 2.0 * rho[ip] * t207 + 4.0 * t115;
        v2rho2[ip] += tv2rho20;
        let t213 = t7 * t23;
        let t214 = t71 * t157;
        let t215 = t138 * t109;
        let t216 = t214 * t215;
        let t223 = t96 * sigma[ip];
        let t227 = t182 * rho[ip];
        let t228 = 1.0 / t227;
        let t234 = (-t118 * t119 * t78 / 9.0 + t88 * t33 * t94 * t223 / 36.0 - t180 * t89 * t228 * t187 / 108.0) * t26;
        let t235 = t234 * t30;
        let t242 = t119 * t77;
        let t245 = t235 * t41 / 24.0 - t133 * t106 / 9.0 + t102 * t135 / 24.0 - t51 * t242 / 9.0;
        let t246 = t74 * t245;
        let t251 = piecewise3::<f64>(t2, 0.0, t70 * t152 * t139 / 10.0 - 3.0 / 10.0 * t213 * t216 + 3.0 / 20.0 * t70 * t72 * t246);
        let tv2rhosigma0 = 2.0 * rho[ip] * t251 + 2.0 * t143;
        v2rhosigma[ip] += tv2rhosigma0;
        let t254 = t138 * t138;
        let t255 = t157 * t254;
        let t259 = t83 * t84;
        let t260 = t86 * t33;
        let t264 = 1.0 / t182;
        let t270 = (-t259 * t260 * t127 / 144.0 + t180 * sigma[ip] * t264 * t187 / 288.0) * t26;
        let t271 = t270 * t30;
        let t276 = t271 * t41 / 24.0 + t132 * t135 / 12.0;
        let t277 = t74 * t276;
        let t282 = piecewise3::<f64>(t2, 0.0, -3.0 / 10.0 * t70 * t72 * t255 + 3.0 / 20.0 * t70 * t72 * t277);
        let tv2sigma20 = 2.0 * rho[ip] * t282;
        v2sigma2[ip] += tv2sigma20;
        let t286 = 1.0 / t21 / t36;
        let t291 = t147 * t71;
        let t301 = t73 * t73;
        let t302 = 1.0 / t301;
        let t303 = t158 * t109;
        let t304 = t302 * t303;
        let t309 = t214 * t109 * t201;
        let t313 = 1.0 / t22 / t124;
        let t314 = t313 * t45;
        let t319 = 1.0 / t21 / t182;
        let t324 = t182 * t75;
        let t325 = 1.0 / t324;
        let t330 = t82 * t82;
        let t331 = t24 * t330;
        let t332 = t89 * t89;
        let t333 = t179 * t332;
        let t334 = t331 * t333;
        let t335 = t182 * t124;
        let t337 = 1.0 / t22 / t335;
        let t338 = t95 * t95;
        let t339 = 1.0 / t338;
        let t341 = t31 * t34;
        let t346 = (-154.0 / 81.0 * t32 * t35 * t314 + 341.0 / 486.0 * t88 * t90 * t319 * t96 - 38.0 / 81.0 * t180 * t181 * t325 * t187 + 2.0 / 243.0 * t334 * t337 * t339 * t341) * t26;
        let t347 = t346 * t30;
        let t354 = t35 * t313;
        let t357 = t347 * t41 / 24.0 - t193 * t106 / 3.0 + 11.0 / 9.0 * t103 * t198 - 154.0 / 81.0 * t52 * t354;
        let t358 = t74 * t357;
        let t363 = piecewise3::<f64>(t2, 0.0, 2.0 / 45.0 * t7 * t20 * t286 * t60 - t70 * t291 * t110 / 10.0 - 3.0 / 5.0 * t70 * t152 * t159 + 3.0 / 10.0 * t70 * t152 * t202 + 9.0 / 10.0 * t70 * t72 * t304 - 9.0 / 10.0 * t213 * t309 + 3.0 / 20.0 * t70 * t72 * t358);
        let tv3rho30 = 2.0 * rho[ip] * t363 + 6.0 * t207;
        v3rho3[ip] += tv3rho30;
        let t370 = t7 * t66;
        let t376 = t71 * t302;
        let t378 = t376 * t138 * t158;
        let t382 = t214 * t245 * t109;
        let t386 = t214 * t138 * t201;
        let t396 = t184 * t187;
        let t400 = t179 * t181;
        let t402 = t182 * t91;
        let t404 = 1.0 / t22 / t402;
        let t410 = (11.0 / 27.0 * t118 * t119 * t165 - 65.0 / 324.0 * t88 * t33 * t171 * t223 + 17.0 / 108.0 * t180 * t396 * t89 - t331 * t400 * t404 * t339 * t341 / 324.0) * t26;
        let t411 = t410 * t30;
        let t422 = t119 * t164;
        let t425 = t411 * t41 / 24.0 - 2.0 / 9.0 * t235 * t106 + 11.0 / 27.0 * t133 * t198 + t192 * t135 / 24.0 - 2.0 / 9.0 * t102 * t242 + 11.0 / 27.0 * t51 * t422;
        let t426 = t74 * t425;
        let t431 = piecewise3::<f64>(t2, 0.0, -t70 * t291 * t139 / 30.0 - 2.0 / 5.0 * t370 * t216 + t70 * t152 * t246 / 5.0 + 9.0 / 10.0 * t213 * t378 - 3.0 / 5.0 * t213 * t382 - 3.0 / 10.0 * t213 * t386 + 3.0 / 20.0 * t70 * t72 * t426);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t431 + 4.0 * t251;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t438 = t376 * t254 * t109;
        let t442 = t214 * t138 * t245;
        let t449 = t214 * t276 * t109;
        let t459 = t179 * t89;
        let t462 = 1.0 / t22 / t324;
        let t468 = (t259 * t260 * t97 / 27.0 - 5.0 / 108.0 * t180 * t228 * t187 * sigma[ip] + t331 * t459 * t462 * t339 * t341 / 864.0) * t26;
        let t469 = t468 * t30;
        let t478 = t469 * t41 / 24.0 - t271 * t106 / 9.0 + t234 * t135 / 12.0 - 2.0 / 9.0 * t132 * t242;
        let t479 = t74 * t478;
        let t484 = piecewise3::<f64>(t2, 0.0, -t70 * t152 * t255 / 5.0 + 9.0 / 10.0 * t213 * t438 - 3.0 / 5.0 * t213 * t442 + t70 * t152 * t277 / 10.0 - 3.0 / 10.0 * t213 * t449 + 3.0 / 20.0 * t70 * t72 * t479);
        let tv3rhosigma20 = 2.0 * rho[ip] * t484 + 2.0 * t282;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t487 = t254 * t138;
        let t488 = t302 * t487;
        let t492 = t138 * t276;
        let t493 = t214 * t492;
        let t500 = t179 * sigma[ip];
        let t503 = 1.0 / t22 / t183;
        let t509 = (t177 * t179 * t264 * t187 / 96.0 - t331 * t500 * t503 * t339 * t341 / 2304.0) * t26;
        let t510 = t509 * t30;
        let t515 = t510 * t41 / 24.0 + t270 * t135 / 8.0;
        let t516 = t74 * t515;
        let t521 = piecewise3::<f64>(t2, 0.0, 9.0 / 10.0 * t70 * t72 * t488 - 9.0 / 10.0 * t213 * t493 + 3.0 / 20.0 * t70 * t72 * t516);
        let tv3sigma30 = 2.0 * rho[ip] * t521;
        v3sigma3[ip] += tv3sigma30;
        let t530 = t286 * t71;
        let t549 = 1.0 / t301 / t55;
        let t550 = t158 * t158;
        let t559 = t201 * t201;
        let t569 = 1.0 / t22 / t92;
        let t585 = t182 * t92;
        let t593 = t24 * t330 * param_alpha;
        let t597 = t182 * t182;
        let t602 = 1.0 / t338 / t44;
        let t604 = t87 * t33;
        let t627 = -14.0 / 135.0 * t7 * t20 / t21 / t75 * t60 + 8.0 / 45.0 * t70 * t530 * t110 + 2.0 / 5.0 * t70 * t291 * t159 - t70 * t291 * t202 / 5.0 + 12.0 / 5.0 * t70 * t152 * t304 - 12.0 / 5.0 * t370 * t309 + 2.0 / 5.0 * t70 * t152 * t358 - 18.0 / 5.0 * t70 * t72 * t549 * t550 + 27.0 / 5.0 * t213 * t376 * t158 * t201 - 9.0 / 10.0 * t70 * t72 * t157 * t559 - 6.0 / 5.0 * t213 * t214 * t109 * t357 + 3.0 / 20.0 * t70 * t72 * t74 * ((2618.0 / 243.0 * t32 * t35 * t569 * t45 - 3047.0 / 486.0 * t88 * t90 / t21 / t227 * t96 + 5126.0 / 729.0 * t180 * t181 / t402 * t187 - 196.0 / 729.0 * t334 / t22 / t585 * t339 * t341 + 16.0 / 2187.0 * t593 * t179 * t332 * sigma[ip] / t21 / t597 / rho[ip] * t602 * t604) * t26 * t30 * t41 / 24.0 - 4.0 / 9.0 * t347 * t106 + 22.0 / 9.0 * t193 * t198 - 616.0 / 81.0 * t103 * t354 + 2618.0 / 243.0 * t52 * t35 * t569);
        let t628 = piecewise3::<f64>(t2, 0.0, t627);
        let tv4rho40 = 2.0 * rho[ip] * t628 + 8.0 * t363;
        v4rho4[ip] += tv4rho40;
        let t715 = t71 * t549;
        let t724 = -t70 * t291 * t246 / 10.0 + 3.0 / 10.0 * t70 * t152 * t426 + 3.0 / 20.0 * t70 * t72 * t74 * ((-154.0 / 81.0 * t118 * t119 * t314 + 253.0 / 162.0 * t88 * t33 * t319 * t223 - 1025.0 / 486.0 * t180 * t325 * t187 * t89 + 89.0 / 972.0 * t331 * t179 * t337 * t339 * t181 * t341 - 2.0 / 729.0 * t593 * t333 / t21 / t597 * t602 * t604) * t26 * t30 * t41 / 24.0 - t411 * t106 / 3.0 + 11.0 / 9.0 * t235 * t198 - 154.0 / 81.0 * t133 * t354 + t346 * t135 / 24.0 - t192 * t242 / 3.0 + 11.0 / 9.0 * t102 * t422 - 154.0 / 81.0 * t51 * t119 * t313) - 3.0 / 10.0 * t213 * t214 * t138 * t357 + 2.0 / 45.0 * t70 * t530 * t139 + t7 * t148 * t216 / 5.0 - 6.0 / 5.0 * t370 * t382 - 3.0 / 5.0 * t370 * t386 + 27.0 / 10.0 * t213 * t376 * t245 * t158 - 9.0 / 10.0 * t213 * t214 * t425 * t109 - 9.0 / 10.0 * t213 * t214 * t245 * t201 + 9.0 / 5.0 * t370 * t378 - 18.0 / 5.0 * t213 * t715 * t138 * t303 + 27.0 / 10.0 * t213 * t376 * t215 * t201;
        let t725 = piecewise3::<f64>(t2, 0.0, t724);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t725 + 6.0 * t431;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t748 = t245 * t245;
        let t817 = t70 * t291 * t255 / 15.0 + 6.0 / 5.0 * t370 * t438 - 4.0 / 5.0 * t370 * t442 - 18.0 / 5.0 * t213 * t715 * t254 * t158 + 18.0 / 5.0 * t213 * t376 * t215 * t245 + 9.0 / 10.0 * t213 * t376 * t254 * t201 - 3.0 / 5.0 * t70 * t72 * t157 * t748 - 3.0 / 5.0 * t213 * t214 * t138 * t425 - t70 * t291 * t277 / 30.0 - 2.0 / 5.0 * t370 * t449 + t70 * t152 * t479 / 5.0 + 9.0 / 10.0 * t213 * t376 * t276 * t158 - 3.0 / 5.0 * t213 * t214 * t478 * t109 - 3.0 / 10.0 * t213 * t214 * t276 * t201 + 3.0 / 20.0 * t70 * t72 * t74 * ((-19.0 / 81.0 * t259 * t260 * t172 + 167.0 / 324.0 * t180 * t396 * sigma[ip] - 25.0 / 864.0 * t331 * t179 * t404 * t339 * t89 * t341 + t593 * t400 / t21 / t182 / t169 * t602 * t604 / 972.0) * t26 * t30 * t41 / 24.0 - 2.0 / 9.0 * t469 * t106 + 11.0 / 27.0 * t271 * t198 + t410 * t135 / 12.0 - 4.0 / 9.0 * t234 * t242 + 22.0 / 27.0 * t132 * t422);
        let t818 = piecewise3::<f64>(t2, 0.0, t817);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t818 + 4.0 * t484;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t859 = t339 * t26;
        let t889 = piecewise3::<f64>(t2, 0.0, 3.0 / 5.0 * t70 * t152 * t488 - 18.0 / 5.0 * t213 * t715 * t487 * t109 + 27.0 / 10.0 * t213 * t376 * t254 * t245 - 3.0 / 5.0 * t370 * t493 + 27.0 / 10.0 * t213 * t376 * t492 * t109 - 9.0 / 10.0 * t213 * t214 * t245 * t276 - 9.0 / 10.0 * t213 * t214 * t138 * t478 + t70 * t152 * t516 / 10.0 - 3.0 / 10.0 * t213 * t214 * t515 * t109 + 3.0 / 20.0 * t70 * t72 * t74 * ((-t177 * t179 * t228 * t187 / 12.0 + 7.0 / 864.0 * t331 * t179 * t462 * t859 * t30 * sigma[ip] * t34 - t593 * t459 / t21 / t585 * t602 * t604 / 2592.0) * t26 * t30 * t41 / 24.0 - t510 * t106 / 9.0 + t468 * t135 / 8.0 - t270 * t242 / 3.0));
        let tv4rhosigma30 = 2.0 * rho[ip] * t889 + 2.0 * t521;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t892 = t254 * t254;
        let t901 = t276 * t276;
        let t935 = piecewise3::<f64>(t2, 0.0, -18.0 / 5.0 * t70 * t72 * t549 * t892 + 27.0 / 5.0 * t213 * t376 * t254 * t276 - 9.0 / 10.0 * t70 * t72 * t157 * t901 - 6.0 / 5.0 * t213 * t214 * t138 * t515 + 3.0 / 20.0 * t70 * t72 * t74 * ((-t331 * t179 * t503 * t859 * t119 / 576.0 + t593 * t500 / t21 / t335 * t602 * t604 / 6912.0) * t26 * t30 * t41 / 24.0 + t509 * t135 / 6.0));
        let tv4sigma40 = 2.0 * rho[ip] * t935;
        v4sigma4[ip] += tv4sigma40;
    }
}
