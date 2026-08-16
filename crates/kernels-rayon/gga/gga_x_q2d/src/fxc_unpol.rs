//! GGA_X_Q2D fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_q2d.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_q2d_fxc_unpol(
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
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = t3 / t4 * t17;
        let t19 = pow_1_3(rho[ip]);
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = M_CBRT2;
        let t27 = t26 * t26;
        let t28 = sigma[ip] * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t19 * t19;
        let t32 = 1.0 / t30 / t29;
        let t34 = t25 * t28 * t32;
        let t36 = 0.804e0 + 5.0 / 972.0 * t34;
        let t39 = 0.1804e1 - 0.646416e0 / t36;
        let t40 = t20 * t20;
        let t42 = 1.0 / t22 / t21;
        let t43 = t40 * t42;
        let t44 = sigma[ip] * sigma[ip];
        let t45 = t44 * t26;
        let t46 = t29 * t29;
        let t47 = t46 * rho[ip];
        let t49 = 1.0 / t19 / t47;
        let t53 = 100.0 - t43 * t45 * t49 / 288.0;
        let t55 = 1.0 / t22;
        let t56 = t40 * t55;
        let t57 = f64::sqrt(sigma[ip]);
        let t60 = 1.0 / t19 / rho[ip];
        let t62 = t56 * t57 * t26 * t60;
        let t63 = f64::powf(t62, 0.35e1);
        let t65 = 1.0 + t34 / 24.0;
        let t68 = t39 * t53 + 0.87153829697982569831e-4 * t63 * t65;
        let t70 = t21 * t21;
        let t71 = 1.0 / t70;
        let t72 = t44 * sigma[ip];
        let t74 = t46 * t46;
        let t75 = 1.0 / t74;
        let t78 = 100.0 + t71 * t72 * t75 / 576.0;
        let t79 = 1.0 / t78;
        let t83 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t19 * t68 * t79);
        let tzk0 = 2.0 * t83;
        zk[ip] += tzk0;
        let t84 = 1.0 / t30;
        let t89 = t36 * t36;
        let t90 = 1.0 / t89;
        let t92 = t90 * t20 * t24;
        let t93 = t29 * rho[ip];
        let t95 = 1.0 / t30 / t93;
        let t100 = t39 * t40;
        let t101 = t100 * t42;
        let t102 = t46 * t29;
        let t104 = 1.0 / t19 / t102;
        let t108 = f64::powf(t62, 0.25e1);
        let t110 = t108 * t65 * t40;
        let t111 = t55 * t57;
        let t114 = t26 / t19 / t29;
        let t118 = t63 * t20;
        let t119 = t118 * t24;
        let t123 = -0.88671604938271604938e-2 * t92 * t28 * t95 * t53 + t101 * t45 * t104 / 54.0 - 0.40671787192391865921e-3 * t110 * t111 * t114 - 0.96837588553313966479e-5 * t119 * t28 * t95;
        let t131 = t3 / t4 / t70 * t17;
        let t133 = 1.0 / t30 / t74;
        let t134 = t133 * t68;
        let t135 = t78 * t78;
        let t136 = 1.0 / t135;
        let t137 = t136 * t72;
        let t142 = piecewise3(t2, 0.0, -t18 * t84 * t68 * t79 / 8.0 - 3.0 / 8.0 * t18 * t19 * t123 * t79 - t131 * t134 * t137 / 192.0);
        let tvrho0 = 2.0 * rho[ip] * t142 + 2.0 * t83;
        vrho[ip] += tvrho0;
        let t145 = t27 * t32;
        let t149 = sigma[ip] * t26;
        let t153 = 1.0 / t57;
        let t154 = t55 * t153;
        let t155 = t26 * t60;
        let t159 = t24 * t27;
        let t163 = 0.33251851851851851852e-2 * t92 * t145 * t53 - t101 * t149 * t49 / 144.0 + 0.1525192019714694972e-3 * t110 * t154 * t155 + 0.3631409570749273743e-5 * t118 * t159 * t32;
        let t168 = t46 * t93;
        let t170 = 1.0 / t30 / t168;
        let t171 = t170 * t68;
        let t172 = t136 * t44;
        let t177 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t19 * t163 * t79 + t131 * t171 * t172 / 512.0);
        let tvsigma0 = 2.0 * rho[ip] * t177;
        vsigma[ip] += tvsigma0;
        let t181 = 1.0 / t30 / rho[ip];
        let t190 = t74 * rho[ip];
        let t192 = 1.0 / t30 / t190;
        let t193 = t192 * t68;
        let t198 = 1.0 / t89 / t36;
        let t200 = t198 * t40 * t42;
        let t202 = 1.0 / t19 / t168;
        let t208 = 1.0 / t30 / t46;
        let t213 = t90 * t71;
        let t214 = t74 * t29;
        let t215 = 1.0 / t214;
        let t219 = t90 * t72;
        let t225 = pow_3_2(t62);
        let t227 = t225 * t65 * t20;
        let t228 = t24 * sigma[ip];
        let t229 = t27 * t208;
        let t233 = 1.0 / t21;
        let t234 = t108 * t233;
        let t235 = t57 * sigma[ip];
        let t236 = 1.0 / t102;
        let t242 = t26 / t19 / t93;
        let t249 = -0.48653829870107876509e-3 * t200 * t45 * t202 * t53 + 0.32512921810699588477e-1 * t92 * t28 * t208 * t53 - 0.19704801097393689986e-2 * t213 * t72 * t215 - 0.20228913839792803583e-4 * t219 * t215 - 19.0 / 162.0 * t101 * t45 * t202 + 0.81343574384783731842e-2 * t227 * t228 * t229 + 0.10845809917971164246e-2 * t234 * t235 * t236 + 0.94900836782247687149e-3 * t110 * t111 * t242 + 0.35507115802881787709e-4 * t119 * t28 * t208;
        let t254 = t133 * t123;
        let t258 = t70 * t70;
        let t262 = t3 / t4 / t258 * t17;
        let t263 = t74 * t74;
        let t264 = t263 * rho[ip];
        let t266 = 1.0 / t30 / t264;
        let t267 = t266 * t68;
        let t269 = 1.0 / t135 / t78;
        let t270 = t44 * t44;
        let t271 = t270 * t44;
        let t272 = t269 * t271;
        let t277 = piecewise3(t2, 0.0, t18 * t181 * t68 * t79 / 12.0 - t18 * t84 * t123 * t79 / 4.0 + 25.0 / 576.0 * t131 * t193 * t137 - 3.0 / 8.0 * t18 * t19 * t249 * t79 - t131 * t254 * t137 / 96.0 - t262 * t267 * t272 / 6912.0);
        let tv2rho20 = 2.0 * rho[ip] * t277 + 4.0 * t142;
        v2rho2[ip] += tv2rho20;
        let t284 = t26 * t104;
        let t285 = t53 * sigma[ip];
        let t289 = t27 * t95;
        let t293 = 1.0 / t190;
        let t297 = t90 * t44;
        let t303 = t159 * t95;
        let t306 = 1.0 / t47;
        let t315 = 0.18245186201290453691e-3 * t200 * t284 * t285 - 0.88671604938271604939e-2 * t92 * t289 * t53 + 0.73893004115226337449e-3 * t213 * t293 * t44 + 0.75858426899223013437e-5 * t297 * t293 + t101 * t149 * t104 / 27.0 - 0.3050384039429389944e-2 * t227 * t303 - 0.40671787192391865921e-3 * t234 * t57 * t306 - 0.2033589359619593296e-3 * t110 * t154 * t114 - 0.9683758855331396648e-5 * t118 * t303;
        let t320 = t133 * t163;
        let t327 = t170 * t123;
        let t332 = 1.0 / t30 / t263;
        let t333 = t332 * t68;
        let t334 = t270 * sigma[ip];
        let t335 = t269 * t334;
        let t340 = piecewise3(t2, 0.0, -t18 * t84 * t163 * t79 / 8.0 - 3.0 / 8.0 * t18 * t19 * t315 * t79 - t131 * t320 * t137 / 192.0 - 23.0 / 1536.0 * t131 * t134 * t172 + t131 * t327 * t172 / 512.0 + t262 * t333 * t335 / 18432.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t340 + 2.0 * t177;
        v2rhosigma[ip] += tv2rhosigma0;
        let t343 = t26 * t49;
        let t350 = t90 * t75;
        let t353 = t42 * t26;
        let t357 = 1.0 / sigma[ip];
        let t358 = t24 * t357;
        let t362 = 1.0 / t46;
        let t366 = 1.0 / t235;
        let t367 = t55 * t366;
        let t371 = -0.68419448254839201342e-4 * t200 * t343 * t53 - 0.27709876543209876543e-3 * t213 * t75 * sigma[ip] - 0.28446910087208630037e-5 * t350 * sigma[ip] - t100 * t353 * t49 / 144.0 + 0.1143894014786021229e-2 * t227 * t358 * t145 + 0.1525192019714694972e-3 * t234 * t362 * t153 - 0.762596009857347486e-4 * t110 * t367 * t155;
        let t376 = t170 * t163;
        let t380 = t74 * t168;
        let t382 = 1.0 / t30 / t380;
        let t383 = t382 * t68;
        let t384 = t269 * t270;
        let t388 = t136 * sigma[ip];
        let t393 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t19 * t371 * t79 + t131 * t376 * t172 / 256.0 - t262 * t383 * t384 / 49152.0 + t131 * t171 * t388 / 256.0);
        let tv2sigma20 = 2.0 * rho[ip] * t393;
        v2sigma2[ip] += tv2sigma20;
    }
}
