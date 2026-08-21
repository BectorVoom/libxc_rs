//! MGGA_X_RSCAN vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rscan.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_rscan_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_alphar: f64,
    param_c2: f64,
    param_d: f64,
    param_k1: f64,
    param_taur: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = t4 / t5 * t18;
        let t20 = pow_1_3(rho[ip]);
        let t21 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t20 * t20;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t35 = t26 * t34;
        let t39 = 100.0 / 6561.0 / param_k1 - 73.0 / 648.0;
        let t40 = t21 * t21;
        let t42 = t23 * t22;
        let t43 = 1.0 / t42;
        let t44 = t39 * t40 * t43;
        let t45 = sigma[ip] * sigma[ip];
        let t46 = t45 * t27;
        let t47 = t30 * t30;
        let t48 = t47 * rho[ip];
        let t50 = 1.0 / t20 / t48;
        let t55 = rmath::exp(-27.0 / 80.0 * t39 * t21 * t25 * t34);
        let t56 = t50 * t55;
        let t60 = rmath::sqrt(146.0);
        let t61 = t60 * t21;
        let t62 = t61 * t25;
        let t65 = t12 * t12;
        let t66 = t65 * t65;
        let t67 = t66 * t12;
        let t68 = t67 * t48;
        let t69 = tau[ip] * t28;
        let t70 = t31 * rho[ip];
        let t71 = 1.0 / t70;
        let t74 = t69 * t71 - t34 / 8.0;
        let t75 = 0.0 < t74;
        let t76 = piecewise3(t75, t74, 0.0);
        let t77 = t76 * t76;
        let t78 = t77 * t76;
        let t79 = t12 * rho[ip];
        let t80 = pow_1_3(t79);
        let t81 = t80 * t80;
        let t84 = t40 * t24;
        let t88 = 3.0 / 40.0 * t27 * t81 * t79 * t84 + param_taur / 2.0;
        let t89 = t88 * t88;
        let t90 = t89 * t88;
        let t91 = 1.0 / t90;
        let t93 = t65 * t12;
        let t94 = t30 * rho[ip];
        let t96 = t80 * t93 * t94;
        let t97 = t28 * t96;
        let t98 = 1.0 / t89;
        let t99 = t77 * t98;
        let t102 = t97 * t99 / 16.0 + param_alphar;
        let t103 = 1.0 / t102;
        let t104 = t78 * t91 * t103;
        let t106 = t68 * t104 / 32.0;
        let t107 = 1.0 - t106;
        let t109 = t107 * t107;
        let t111 = rmath::exp(-t109 / 2.0);
        let t114 = 7.0 / 12960.0 * t62 * t34 + t60 * t107 * t111 / 100.0;
        let t115 = t114 * t114;
        let t116 = param_k1 + 5.0 / 972.0 * t35 + t44 * t46 * t56 / 288.0 + t115;
        let t121 = 1.0 + param_k1 * (1.0 - param_k1 / t116);
        let t122 = t106 <= 2.5;
        let t123 = 2.5 < t106;
        let t124 = piecewise3(t123, 2.5, t106);
        let t126 = t124 * t124;
        let t128 = t126 * t124;
        let t130 = t126 * t126;
        let t132 = t130 * t124;
        let t134 = t130 * t126;
        let t139 = piecewise3(t123, t106, 2.5);
        let t140 = 1.0 - t139;
        let t143 = rmath::exp(param_c2 / t140);
        let t145 = piecewise3(t122, 1.0 - 0.667 * t124 - 0.4445555 * t126 - 0.663086601049 * t128 + 1.45129704449 * t130 - 0.887998041597 * t132 + 0.234528941479 * t134 - 0.023185843322 * t130 * t128, -param_d * t143);
        let t146 = 1.0 - t145;
        let t149 = t121 * t146 + 1.174 * t145;
        let t151 = rmath::sqrt(3.0);
        let t152 = 1.0 / t23;
        let t153 = t40 * t152;
        let t154 = rmath::sqrt(sigma[ip]);
        let t155 = t154 * t27;
        let t157 = 1.0 / t20 / rho[ip];
        let t159 = t153 * t155 * t157;
        let t160 = rmath::sqrt(t159);
        let t164 = rmath::exp(-9.8958 * t151 / t160);
        let t165 = 1.0 - t164;
        let t169 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t149 * t165);
        let tzk0 = 2.0 * t169;
        zk[ip] += tzk0;
        let t170 = 1.0 / t31;
        let t175 = param_k1 * param_k1;
        let t176 = t116 * t116;
        let t178 = t175 / t176;
        let t180 = 1.0 / t31 / t94;
        let t181 = t29 * t180;
        let t184 = t47 * t30;
        let t186 = 1.0 / t20 / t184;
        let t187 = t186 * t55;
        let t191 = t39 * t39;
        let t192 = t22 * t22;
        let t193 = 1.0 / t192;
        let t194 = t191 * t193;
        let t195 = t45 * sigma[ip];
        let t196 = t47 * t47;
        let t197 = t196 * rho[ip];
        let t198 = 1.0 / t197;
        let t205 = t67 * t47;
        let t208 = t68 * t77;
        let t209 = t91 * t103;
        let t214 = piecewise3(t75, -5.0 / 3.0 * t69 * t33 + t181 / 3.0, 0.0);
        let t215 = t209 * t214;
        let t218 = t66 * t65;
        let t219 = t218 * t48;
        let t220 = t89 * t89;
        let t221 = 1.0 / t220;
        let t222 = t78 * t221;
        let t224 = t103 * t27;
        let t226 = t81 * t40 * t24;
        let t227 = t224 * t226;
        let t230 = t68 * t78;
        let t231 = t102 * t102;
        let t232 = 1.0 / t231;
        let t233 = t91 * t232;
        let t235 = t80 * t65 * t30;
        let t236 = t28 * t235;
        let t240 = t76 * t98;
        let t244 = t205 * t77;
        let t246 = t91 * t40 * t24;
        let t249 = 5.0 / 24.0 * t236 * t99 * t12 + t97 * t240 * t214 / 8.0 - t244 * t246 / 32.0;
        let t250 = t233 * t249;
        let t253 = -5.0 / 32.0 * t205 * t104 - 3.0 / 32.0 * t208 * t215 + 3.0 / 256.0 * t219 * t222 * t227 + t230 * t250 / 32.0;
        let t257 = t60 * t109;
        let t258 = t253 * t111;
        let t261 = -7.0 / 4860.0 * t62 * t181 + t60 * t253 * t111 / 100.0 - t257 * t258 / 100.0;
        let t264 = -10.0 / 729.0 * t26 * t181 - t44 * t46 * t187 / 54.0 + 3.0 / 80.0 * t194 * t195 * t198 * t55 + 2.0 * t114 * t261;
        let t265 = t264 * t146;
        let t267 = -t253;
        let t268 = piecewise3(t123, 0.0, t267);
        let t270 = t124 * t268;
        let t272 = t126 * t268;
        let t274 = t128 * t268;
        let t276 = t130 * t268;
        let t278 = t132 * t268;
        let t283 = param_d * param_c2;
        let t284 = t140 * t140;
        let t285 = 1.0 / t284;
        let t286 = piecewise3(t123, t267, 0.0);
        let t290 = piecewise3(t122, -0.667 * t268 - 0.889111 * t270 - 1.989259803147 * t272 + 5.80518817796 * t274 - 4.439990207985 * t276 + 1.407173648874 * t278 - 0.162300903254 * t134 * t268, -t283 * t285 * t286 * t143);
        let t293 = t178 * t265 - t121 * t290 + 1.174 * t290;
        let t298 = rmath::pow(3.0, 1.0 / 6.0);
        let t299 = t298 * t298;
        let t300 = t299 * t299;
        let t302 = t300 * t298 * t18;
        let t303 = 1.0 / t30;
        let t304 = t303 * t149;
        let t306 = 1.0 / t160 / t159;
        let t308 = t302 * t304 * t306;
        let t310 = t153 * t155 * t164;
        let t314 = piecewise3(t3, 0.0, -t19 * t170 * t149 * t165 / 8.0 - 3.0 / 8.0 * t19 * t20 * t293 * t165 - 1.6891736332904388 * t308 * t310);
        let tvrho0 = 2.0 * rho[ip] * t314 + 2.0 * t169;
        vrho[ip] += tvrho0;
        let t317 = t28 * t33;
        let t320 = sigma[ip] * t27;
        let t324 = 1.0 / t196;
        let t329 = t25 * t28;
        let t334 = piecewise3(t75, -t317 / 8.0, 0.0);
        let t335 = t209 * t334;
        let t338 = t77 * t77;
        let t340 = 1.0 / t220 / t88;
        let t341 = t338 * t340;
        let t342 = t68 * t341;
        let t343 = t232 * t28;
        let t344 = t96 * t334;
        let t345 = t343 * t344;
        let t348 = -3.0 / 32.0 * t208 * t335 + t342 * t345 / 256.0;
        let t349 = t60 * t348;
        let t352 = t348 * t111;
        let t355 = 7.0 / 12960.0 * t61 * t329 * t33 + t349 * t111 / 100.0 - t257 * t352 / 100.0;
        let t358 = 5.0 / 972.0 * t26 * t317 + t44 * t320 * t56 / 144.0 - 9.0 / 640.0 * t194 * t45 * t324 * t55 + 2.0 * t114 * t355;
        let t359 = t358 * t146;
        let t361 = -t348;
        let t362 = piecewise3(t123, 0.0, t361);
        let t364 = t124 * t362;
        let t366 = t126 * t362;
        let t368 = t128 * t362;
        let t370 = t130 * t362;
        let t372 = t132 * t362;
        let t377 = piecewise3(t123, t361, 0.0);
        let t381 = piecewise3(t122, -0.667 * t362 - 0.889111 * t364 - 1.989259803147 * t366 + 5.80518817796 * t368 - 4.439990207985 * t370 + 1.407173648874 * t372 - 0.162300903254 * t134 * t362, -t283 * t285 * t377 * t143);
        let t384 = t178 * t359 - t121 * t381 + 1.174 * t381;
        let t389 = 1.0 / rho[ip];
        let t390 = t389 * t149;
        let t392 = t302 * t390 * t306;
        let t393 = 1.0 / t154;
        let t396 = t153 * t393 * t27 * t164;
        let t400 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t384 * t165 + 0.6334401124839145 * t392 * t396);
        let tvsigma0 = 2.0 * rho[ip] * t400;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t403 = piecewise3(t75, t28 * t71, 0.0);
        let t404 = t209 * t403;
        let t407 = t96 * t403;
        let t408 = t343 * t407;
        let t411 = -3.0 / 32.0 * t208 * t404 + t342 * t408 / 256.0;
        let t412 = t60 * t411;
        let t414 = t411 * t111;
        let t417 = t412 * t111 / 100.0 - t257 * t414 / 100.0;
        let t418 = t114 * t417;
        let t422 = -t411;
        let t423 = piecewise3(t123, 0.0, t422);
        let t425 = t124 * t423;
        let t427 = t126 * t423;
        let t429 = t128 * t423;
        let t431 = t130 * t423;
        let t433 = t132 * t423;
        let t438 = piecewise3(t123, t422, 0.0);
        let t442 = piecewise3(t122, -0.667 * t423 - 0.889111 * t425 - 1.989259803147 * t427 + 5.80518817796 * t429 - 4.439990207985 * t431 + 1.407173648874 * t433 - 0.162300903254 * t134 * t423, -t283 * t285 * t438 * t143);
        let t445 = 2.0 * t178 * t418 * t146 - t121 * t442 + 1.174 * t442;
        let t450 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t445 * t165);
        let tvtau0 = 2.0 * rho[ip] * t450;
        vtau[ip] += tvtau0;
    }
}
