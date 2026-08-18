//! GGA_X_FD_LB94 lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_fd_lb94.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::integrate::{xc_integrate_func0, xc_integrate_func1};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_fd_lb94_lxc_unpol(
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
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = t20 * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = 1.0 / t23;
        let t25 = t21 * t24;
        let t26 = f64::sqrt(sigma[ip]);
        let t27 = t25 * t26;
        let t28 = M_CBRT2;
        let t30 = 1.0 / t18 / rho[ip];
        let t31 = t28 * t30;
        let t35 = t25 * t26 * t28 * t30 / 12.0;
        let t36 = xc_integrate_func0(t35, param_beta);
        let t37 = f64::ln(t35);
        let t39 = xc_integrate_func1(t35, param_beta);
        let t40 = t36 * t37 - t39;
        let t41 = t31 * t40;
        let t44 = 1.0 - t27 * t41 / 12.0;
        let t48 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t44);
        let tzk0 = 2.0 * t48;
        zk[ip] += tzk0;
        let t49 = t18 * t18;
        let t51 = t17 / t49;
        let t55 = rho[ip] * rho[ip];
        let t57 = 1.0 / t18 / t55;
        let t58 = t28 * t57;
        let t59 = t58 * t40;
        let t61 = t58 * t36;
        let t64 = t27 * t59 / 9.0 + t27 * t61 / 9.0;
        let t69 = piecewise3(t2, 0.0, -t6 * t51 * t44 / 8.0 - 3.0 / 8.0 * t6 * t19 * t64);
        let tvrho0 = 2.0 * rho[ip] * t69 + 2.0 * t48;
        vrho[ip] += tvrho0;
        let t72 = 1.0 / t26;
        let t73 = t25 * t72;
        let t75 = t31 * t36;
        let t78 = -t73 * t41 / 24.0 - t73 * t75 / 24.0;
        let t82 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t78);
        let tvsigma0 = 2.0 * rho[ip] * t82;
        vsigma[ip] += tvsigma0;
        let t87 = t17 / t49 / rho[ip];
        let t94 = t55 * rho[ip];
        let t96 = 1.0 / t18 / t94;
        let t97 = t28 * t96;
        let t98 = t97 * t40;
        let t101 = t97 * t36;
        let t104 = t23 * t23;
        let t105 = 1.0 / t104;
        let t106 = t20 * t105;
        let t107 = t106 * sigma[ip];
        let t108 = t55 * t55;
        let t110 = 1.0 / t49 / t108;
        let t111 = t110 * param_beta;
        let t112 = t28 * t28;
        let t113 = param_beta * t112;
        let t114 = t113 * t21;
        let t115 = t24 * t26;
        let t116 = t112 * t21;
        let t121 = t105 * sigma[ip];
        let t123 = 1.0 / t49 / t55;
        let t127 = 3.0 * t28 * t20 * t121 * t123 + 36.0;
        let t128 = f64::sqrt(t127);
        let t130 = t116 * t115 * t30 / 12.0 + t128 / 6.0;
        let t131 = f64::ln(t130);
        let t132 = t30 * t131;
        let t136 = 1.0 + t114 * t115 * t132 / 4.0;
        let t137 = 1.0 / t136;
        let t138 = t111 * t137;
        let t141 = -7.0 / 27.0 * t27 * t98 - 11.0 / 27.0 * t27 * t101 + t107 * t138 / 9.0;
        let t146 = piecewise3(t2, 0.0, t6 * t87 * t44 / 12.0 - t6 * t51 * t64 / 4.0 - 3.0 / 8.0 * t6 * t19 * t141);
        let tv2rho20 = 2.0 * rho[ip] * t146 + 4.0 * t69;
        v2rho2[ip] += tv2rho20;
        let t157 = 1.0 / t49 / t94;
        let t158 = t157 * param_beta;
        let t159 = t158 * t137;
        let t162 = t73 * t59 / 18.0 + t73 * t61 / 9.0 - t106 * t159 / 24.0;
        let t167 = piecewise3(t2, 0.0, -t6 * t51 * t78 / 8.0 - 3.0 / 8.0 * t6 * t19 * t162);
        let tv2rhosigma0 = 2.0 * rho[ip] * t167 + 2.0 * t82;
        v2rhosigma[ip] += tv2rhosigma0;
        let t171 = 1.0 / t26 / sigma[ip];
        let t172 = t25 * t171;
        let t175 = 1.0 / sigma[ip];
        let t176 = t106 * t175;
        let t177 = t123 * param_beta;
        let t178 = t177 * t137;
        let t181 = t172 * t41 / 48.0 + t176 * t178 / 64.0;
        let t185 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t181);
        let tv2sigma20 = 2.0 * rho[ip] * t185;
        v2sigma2[ip] += tv2sigma20;
        let t188 = t17 * t123;
        let t200 = t28 / t18 / t108;
        let t201 = t200 * t40;
        let t204 = t200 * t36;
        let t207 = t108 * rho[ip];
        let t210 = 1.0 / t49 / t207 * param_beta;
        let t211 = t210 * t137;
        let t214 = t136 * t136;
        let t215 = 1.0 / t214;
        let t216 = t57 * t131;
        let t220 = t113 * t25;
        let t221 = t26 * t30;
        let t226 = 1.0 / t128 * t28;
        let t227 = t226 * t20;
        let t231 = -t116 * t115 * t57 / 9.0 - 2.0 / 3.0 * t227 * t121 * t157;
        let t232 = 1.0 / t130;
        let t233 = t231 * t232;
        let t237 = -t114 * t115 * t216 / 3.0 + t220 * t221 * t233 / 4.0;
        let t238 = t215 * t237;
        let t242 = 70.0 / 81.0 * t27 * t201 + 46.0 / 27.0 * t27 * t204 - 25.0 / 27.0 * t107 * t211 - t107 * t111 * t238 / 9.0;
        let t247 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t188 * t44 + t6 * t87 * t64 / 4.0 - 3.0 / 8.0 * t6 * t51 * t141 - 3.0 / 8.0 * t6 * t19 * t242);
        let tv3rho30 = 2.0 * rho[ip] * t247 + 6.0 * t146;
        v3rho3[ip] += tv3rho30;
        let t263 = t106 * t157;
        let t264 = param_beta * t215;
        let t265 = t264 * t237;
        let t268 = -7.0 / 54.0 * t73 * t98 - t73 * t101 / 3.0 + 19.0 / 72.0 * t106 * t138 + t263 * t265 / 24.0;
        let t273 = piecewise3(t2, 0.0, t6 * t87 * t78 / 12.0 - t6 * t51 * t162 / 4.0 - 3.0 / 8.0 * t6 * t19 * t268);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t273 + 4.0 * t167;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t285 = t177 * t238;
        let t288 = -t172 * t59 / 36.0 - t172 * t61 / 36.0 - t176 * t159 / 24.0 - t176 * t285 / 64.0;
        let t293 = piecewise3(t2, 0.0, -t6 * t51 * t181 / 8.0 - 3.0 / 8.0 * t6 * t19 * t288);
        let tv3rhosigma20 = 2.0 * rho[ip] * t293 + 2.0 * t185;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t296 = sigma[ip] * sigma[ip];
        let t299 = t25 / t26 / t296;
        let t305 = t106 / t296;
        let t308 = t24 * t72;
        let t318 = t116 * t308 * t30 / 24.0 + t226 * t106 * t123 / 4.0;
        let t319 = t318 * t232;
        let t323 = t114 * t308 * t132 / 8.0 + t220 * t221 * t319 / 4.0;
        let t324 = t215 * t323;
        let t325 = t177 * t324;
        let t328 = -t299 * t41 / 32.0 + t299 * t75 / 96.0 - t305 * t178 / 64.0 - t176 * t325 / 64.0;
        let t332 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t328);
        let tv3sigma30 = 2.0 * rho[ip] * t332;
        v3sigma3[ip] += tv3sigma30;
        let t349 = 1.0 / t18 / t207;
        let t350 = t28 * t349;
        let t357 = t108 * t55;
        let t368 = 1.0 / t214 / t136;
        let t369 = t237 * t237;
        let t370 = t368 * t369;
        let t378 = t26 * t57;
        let t387 = 1.0 / t128 / t127 * t112;
        let t388 = t387 * t21;
        let t390 = 1.0 / t23 / t22;
        let t406 = t231 * t231;
        let t407 = t130 * t130;
        let t408 = 1.0 / t407;
        let t413 = 7.0 / 9.0 * t114 * t115 * t96 * t131 - 2.0 / 3.0 * t220 * t378 * t233 + t220 * t221 * (7.0 / 27.0 * t116 * t115 * t96 - 8.0 / 3.0 * t388 * t390 * t296 / t18 / t108 / t94 + 22.0 / 9.0 * t227 * t121 * t110) * t232 / 4.0 - t220 * t221 * t406 * t408 / 4.0;
        let t414 = t215 * t413;
        let t423 = piecewise3(t2, 0.0, 10.0 / 27.0 * t6 * t17 * t157 * t44 - 5.0 / 9.0 * t6 * t188 * t64 + t6 * t87 * t141 / 2.0 - t6 * t51 * t242 / 2.0 - 3.0 / 8.0 * t6 * t19 * (-910.0 / 243.0 * t27 * t350 * t40 - 2074.0 / 243.0 * t27 * t350 * t36 + 563.0 / 81.0 * t107 / t49 / t357 * param_beta * t137 + 13.0 / 9.0 * t107 * t210 * t238 + 2.0 / 9.0 * t107 * t111 * t370 - t107 * t111 * t414 / 9.0));
        let tv4rho40 = 2.0 * rho[ip] * t423 + 8.0 * t247;
        v4rho4[ip] += tv4rho40;
        let t445 = param_beta * t368;
        let t457 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t188 * t78 + t6 * t87 * t162 / 4.0 - 3.0 / 8.0 * t6 * t51 * t268 - 3.0 / 8.0 * t6 * t19 * (35.0 / 81.0 * t73 * t201 + 104.0 / 81.0 * t73 * t204 - 169.0 / 108.0 * t106 * t211 - 5.0 / 12.0 * t106 * t110 * t265 - t263 * t445 * t369 / 12.0 + t263 * t264 * t413 / 24.0));
        let tv4rho3sigma0 = 2.0 * rho[ip] * t457 + 6.0 * t273;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t487 = piecewise3(t2, 0.0, t6 * t87 * t181 / 12.0 - t6 * t51 * t288 / 4.0 - 3.0 / 8.0 * t6 * t19 * (7.0 / 108.0 * t172 * t98 + 11.0 / 108.0 * t172 * t101 + t176 * t138 / 8.0 + t176 * t158 * t238 / 12.0 + t176 * t177 * t370 / 32.0 - t176 * t177 * t414 / 64.0));
        let tv4rho2sigma20 = 2.0 * rho[ip] * t487 + 4.0 * t293;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t513 = t72 * t30;
        let t550 = piecewise3(t2, 0.0, -t6 * t51 * t328 / 8.0 - 3.0 / 8.0 * t6 * t19 * (t299 * t59 / 24.0 + t299 * t61 / 36.0 + 5.0 / 96.0 * t305 * t159 + t305 * t285 / 64.0 + t176 * t158 * t324 / 24.0 + t106 * t175 * t123 * t445 * t323 * t237 / 32.0 - t176 * t177 * t215 * (-t114 * t308 * t216 / 6.0 + t220 * t513 * t233 / 8.0 - t220 * t378 * t319 / 3.0 + t220 * t221 * (-t116 * t308 * t57 / 18.0 + t388 * t390 / t18 / t357 * sigma[ip] - 2.0 / 3.0 * t226 * t263) * t232 / 4.0 - t220 * t221 * t318 * t408 * t231 / 4.0) / 64.0));
        let tv4rhosigma30 = 2.0 * rho[ip] * t550 + 2.0 * t332;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t553 = t296 * sigma[ip];
        let t556 = t25 / t26 / t553;
        let t567 = t323 * t323;
        let t572 = t24 * t171;
        let t591 = t318 * t318;
        let t605 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * (5.0 / 64.0 * t556 * t41 - t556 * t75 / 24.0 + 7.0 / 256.0 * t106 / t553 * t178 + t305 * t325 / 32.0 + t176 * t177 * t368 * t567 / 32.0 - t176 * t177 * t215 * (-t114 * t572 * t132 / 16.0 + t220 * t513 * t319 / 4.0 + t220 * t221 * (-t116 * t572 * t30 / 48.0 - 3.0 / 8.0 * t387 * t21 * t390 * t349) * t232 / 4.0 - t220 * t221 * t591 * t408 / 4.0) / 64.0));
        let tv4sigma40 = 2.0 * rho[ip] * t605;
        v4sigma4[ip] += tv4sigma40;
    }
}
