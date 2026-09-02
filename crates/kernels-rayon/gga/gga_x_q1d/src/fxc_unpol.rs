//! GGA_X_Q1D fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_q1d.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_q1d_fxc_unpol(
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
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = M_CBRT2;
        let t27 = t26 * t26;
        let t28 = sigma[ip] * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t33 = t28 * t32;
        let t34 = t25 * t33;
        let t36 = 0.804 + 5.0 / 972.0 * t34;
        let t38 = 0.646416 / t36;
        let t40 = t20 * t20;
        let t42 = 1.0 / t22 / t21;
        let t43 = t40 * t42;
        let t44 = sigma[ip] * sigma[ip];
        let t45 = t44 * t26;
        let t46 = t29 * t29;
        let t47 = t46 * rho[ip];
        let t49 = 1.0 / t18 / t47;
        let t52 = t43 * t45 * t49 / 288.0;
        let t53 = t34 / 24.0 + t52;
        let t54 = t21 * t21;
        let t55 = 1.0 / t54;
        let t56 = t44 * sigma[ip];
        let t57 = t55 * t56;
        let t58 = t46 * t46;
        let t59 = 1.0 / t58;
        let t62 = 1.0 + t52 + t57 * t59 / 576.0;
        let t63 = 1.0 / t62;
        let t64 = t53 * t63;
        let t66 = (1.804 - t38) * t20;
        let t67 = t66 * t24;
        let t70 = -t67 * t33 / 24.0 + 0.06525;
        let t72 = 1.804 - t38 + t64 * t70;
        let t76 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t72);
        let tzk0 = 2.0 * t76;
        zk[ip] += tzk0;
        let t78 = t17 / t30;
        let t82 = t36 * t36;
        let t83 = 1.0 / t82;
        let t84 = t83 * t20;
        let t85 = t84 * t24;
        let t86 = t29 * rho[ip];
        let t88 = 1.0 / t30 / t86;
        let t89 = t28 * t88;
        let t94 = t46 * t29;
        let t96 = 1.0 / t18 / t94;
        let t97 = t45 * t96;
        let t99 = t43 * t97 / 54.0;
        let t100 = -t25 * t89 / 9.0 - t99;
        let t101 = t100 * t63;
        let t103 = t62 * t62;
        let t104 = 1.0 / t103;
        let t105 = t53 * t104;
        let t106 = t58 * rho[ip];
        let t107 = 1.0 / t106;
        let t110 = -t99 - t57 * t107 / 72.0;
        let t111 = t70 * t110;
        let t113 = t83 * t40;
        let t114 = t113 * t42;
        let t119 = 0.0007389300411522634 * t114 * t97 + t67 * t89 / 9.0;
        let t121 = -0.00886716049382716 * t85 * t89 + t101 * t70 - t105 * t111 + t64 * t119;
        let t126 = piecewise3(t2, 0.0, -t6 * t78 * t72 / 8.0 - 3.0 / 8.0 * t6 * t19 * t121);
        let tvrho0 = 2.0 * rho[ip] * t126 + 2.0 * t76;
        vrho[ip] += tvrho0;
        let t129 = t24 * t27;
        let t130 = t129 * t32;
        let t137 = sigma[ip] * t26 * t49;
        let t139 = t43 * t137 / 144.0;
        let t140 = t25 * t27 * t32 / 24.0 + t139;
        let t141 = t140 * t63;
        let t143 = t55 * t44;
        let t146 = t139 + t143 * t59 / 192.0;
        let t147 = t70 * t146;
        let t153 = -0.00027709876543209876 * t114 * t137 - t66 * t130 / 24.0;
        let t155 = 0.0033251851851851854 * t84 * t130 + t141 * t70 - t105 * t147 + t64 * t153;
        let t159 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t155);
        let tvsigma0 = 2.0 * rho[ip] * t159;
        vsigma[ip] += tvsigma0;
        let t164 = t17 / t30 / rho[ip];
        let t172 = 1.0 / t82 / t36;
        let t173 = t172 * t40;
        let t174 = t173 * t42;
        let t175 = t46 * t86;
        let t177 = 1.0 / t18 / t175;
        let t178 = t45 * t177;
        let t182 = 1.0 / t30 / t46;
        let t183 = t28 * t182;
        let t189 = 19.0 / 162.0 * t43 * t178;
        let t190 = 11.0 / 27.0 * t25 * t183 + t189;
        let t191 = t190 * t63;
        let t193 = t100 * t104;
        let t199 = 1.0 / t103 / t62;
        let t200 = t53 * t199;
        let t201 = t110 * t110;
        let t202 = t70 * t201;
        let t205 = t119 * t110;
        let t208 = t58 * t29;
        let t209 = 1.0 / t208;
        let t212 = t189 + t57 * t209 / 8.0;
        let t213 = t70 * t212;
        let t215 = t172 * t55;
        let t223 = 0.00024326914935053937 * t215 * t56 * t209 - 0.006650370370370371 * t114 * t178 - 11.0 / 27.0 * t67 * t183;
        let t225 = -0.00048653829870107875 * t174 * t178 + 0.03251292181069959 * t85 * t183 + t191 * t70 - 2.0 * t193 * t111 + 2.0 * t101 * t119 + 2.0 * t200 * t202 - 2.0 * t105 * t205 - t105 * t213 + t64 * t223;
        let t230 = piecewise3(t2, 0.0, t6 * t164 * t72 / 12.0 - t6 * t78 * t121 / 4.0 - 3.0 / 8.0 * t6 * t19 * t225);
        let tv2rho20 = 2.0 * rho[ip] * t230 + 4.0 * t126;
        v2rho2[ip] += tv2rho20;
        let t236 = t26 * t96;
        let t237 = t236 * sigma[ip];
        let t240 = t129 * t88;
        let t247 = t43 * t237 / 27.0;
        let t248 = -t25 * t27 * t88 / 9.0 - t247;
        let t249 = t248 * t63;
        let t251 = t140 * t104;
        let t255 = t147 * t110;
        let t258 = t119 * t146;
        let t262 = -t247 - t143 * t107 / 24.0;
        let t263 = t70 * t262;
        let t266 = t153 * t110;
        let t275 = -9.122593100645226e-05 * t215 * t107 * t44 + 0.00221679012345679 * t114 * t237 + t66 * t240 / 9.0;
        let t277 = 0.00018245186201290453 * t174 * t237 - 0.00886716049382716 * t84 * t240 + t249 * t70 - t251 * t111 + t141 * t119 - t193 * t147 + 2.0 * t200 * t255 - t105 * t258 - t105 * t263 + t101 * t153 - t105 * t266 + t64 * t275;
        let t282 = piecewise3(t2, 0.0, -t6 * t78 * t155 / 8.0 - 3.0 / 8.0 * t6 * t19 * t277);
        let tv2rhosigma0 = 2.0 * rho[ip] * t282 + 2.0 * t159;
        v2rhosigma[ip] += tv2rhosigma0;
        let t285 = t42 * t26;
        let t286 = t285 * t49;
        let t289 = t43 * t26;
        let t290 = t49 * t63;
        let t298 = t146 * t146;
        let t299 = t70 * t298;
        let t302 = t153 * t146;
        let t306 = t43 * t26 * t49;
        let t308 = t55 * sigma[ip];
        let t311 = t306 / 144.0 + t308 * t59 / 96.0;
        let t312 = t70 * t311;
        let t319 = 3.42097241274196e-05 * t215 * t59 * sigma[ip] - 0.0005541975308641975 * t113 * t286;
        let t321 = -6.84194482548392e-05 * t173 * t286 + t289 * t290 * t70 / 144.0 - 2.0 * t251 * t147 + 2.0 * t141 * t153 + 2.0 * t200 * t299 - 2.0 * t105 * t302 - t105 * t312 + t64 * t319;
        let t325 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t321);
        let tv2sigma20 = 2.0 * rho[ip] * t325;
        v2sigma2[ip] += tv2sigma20;
    }
}
