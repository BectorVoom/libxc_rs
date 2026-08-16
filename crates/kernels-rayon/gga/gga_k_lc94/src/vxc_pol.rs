//! GGA_K_LC94 vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lc94.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_lc94_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_a: f64,
    param_alpha: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_expo: f64,
    param_f: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = t2 * t2;
        let t4 = M_CBRTPI;
        let t6 = t3 * t4 * M_PI;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * t22;
        let t24 = t23 * zeta_threshold;
        let t25 = pow_1_3(t20);
        let t26 = t25 * t25;
        let t28 = piecewise3(t21, t24, t26 * t20);
        let t29 = pow_1_3(t7);
        let t30 = t29 * t29;
        let t31 = t28 * t30;
        let t32 = M_CBRT6;
        let t33 = param_alpha * t32;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = 1.0 / t36;
        let t38 = t37 * sigma0;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t44 = t38 * t43;
        let t47 = f64::exp(-t33 * t44 / 24.0);
        let t50 = (param_d * t47 + param_c) * t32;
        let t53 = t32 * t32;
        let t54 = 1.0 / t35;
        let t55 = t53 * t54;
        let t56 = f64::sqrt(sigma0);
        let t58 = 1.0 / t40 / rho0;
        let t62 = f64::powf(t55 * t56 * t58 / 12.0, param_expo);
        let t63 = param_f * t62;
        let t64 = t50 * t44 / 24.0 - t63;
        let t65 = t55 * t56;
        let t67 = param_b * t53;
        let t72 = f64::ln(t67 * t54 * t56 * t58 / 12.0 + f64::sqrt(pow_2(t67 * t54 * t56 * t58 / 12.0) + 1.0));
        let t73 = t58 * param_a * t72;
        let t76 = 1.0 + t65 * t73 / 12.0 + t63;
        let t77 = 1.0 / t76;
        let t79 = t64 * t77 + 1.0;
        let t83 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t79);
        let t84 = rho1 <= dens_threshold;
        let t85 = -t17;
        let t87 = piecewise5(t15, t12, t11, t16, t85 * t8);
        let t88 = 1.0 + t87;
        let t89 = t88 <= zeta_threshold;
        let t90 = pow_1_3(t88);
        let t91 = t90 * t90;
        let t93 = piecewise3(t89, t24, t91 * t88);
        let t94 = t93 * t30;
        let t95 = t37 * sigma2;
        let t96 = rho1 * rho1;
        let t97 = pow_1_3(rho1);
        let t98 = t97 * t97;
        let t100 = 1.0 / t98 / t96;
        let t101 = t95 * t100;
        let t104 = f64::exp(-t33 * t101 / 24.0);
        let t107 = (param_d * t104 + param_c) * t32;
        let t110 = f64::sqrt(sigma2);
        let t112 = 1.0 / t97 / rho1;
        let t116 = f64::powf(t55 * t110 * t112 / 12.0, param_expo);
        let t117 = param_f * t116;
        let t118 = t107 * t101 / 24.0 - t117;
        let t119 = t55 * t110;
        let t125 = f64::ln(t67 * t54 * t110 * t112 / 12.0 + f64::sqrt(pow_2(t67 * t54 * t110 * t112 / 12.0) + 1.0));
        let t126 = t112 * param_a * t125;
        let t129 = 1.0 + t119 * t126 / 12.0 + t117;
        let t130 = 1.0 / t129;
        let t132 = t118 * t130 + 1.0;
        let t136 = piecewise3(t84, 0.0, 3.0 / 20.0 * t6 * t94 * t132);
        let tzk0 = t83 + t136;
        zk[ip] += tzk0;
        let t137 = t7 * t7;
        let t138 = 1.0 / t137;
        let t139 = t17 * t138;
        let t141 = piecewise5(t11, 0.0, t15, 0.0, t8 - t139);
        let t144 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t141);
        let t145 = t144 * t30;
        let t149 = 1.0 / t29;
        let t150 = t28 * t149;
        let t153 = t6 * t150 * t79 / 10.0;
        let t155 = param_d * param_alpha * t53;
        let t157 = 1.0 / t35 / t34;
        let t158 = sigma0 * sigma0;
        let t159 = t157 * t158;
        let t160 = t39 * t39;
        let t161 = t160 * t39;
        let t163 = 1.0 / t40 / t161;
        let t168 = t39 * rho0;
        let t170 = 1.0 / t41 / t168;
        let t174 = 1.0 / rho0;
        let t177 = 4.0 / 3.0 * t63 * param_expo * t174;
        let t178 = t155 * t159 * t163 * t47 / 216.0 - t50 * t38 * t170 / 9.0 + t177;
        let t180 = t76 * t76;
        let t181 = 1.0 / t180;
        let t182 = t64 * t181;
        let t186 = 1.0 / t40 / t39 * param_a * t72;
        let t189 = t32 * t37;
        let t190 = t189 * sigma0;
        let t192 = param_b * param_b;
        let t193 = t192 * t32;
        let t196 = 6.0 * t193 * t44 + 144.0;
        let t197 = f64::sqrt(t196);
        let t198 = 1.0 / t197;
        let t199 = param_b * t198;
        let t200 = t170 * param_a * t199;
        let t203 = -t65 * t186 / 9.0 - 2.0 / 3.0 * t190 * t200 - t177;
        let t205 = t178 * t77 - t182 * t203;
        let t210 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t145 * t79 + t153 + 3.0 / 20.0 * t6 * t31 * t205);
        let t211 = t85 * t138;
        let t213 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t211);
        let t216 = piecewise3(t89, 0.0, 5.0 / 3.0 * t91 * t213);
        let t217 = t216 * t30;
        let t221 = t93 * t149;
        let t224 = t6 * t221 * t132 / 10.0;
        let t226 = piecewise3(t84, 0.0, 3.0 / 20.0 * t6 * t217 * t132 + t224);
        let tvrho0 = t83 + t136 + t7 * (t210 + t226);
        vrho[ip * 2] += tvrho0;
        let t230 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t139);
        let t233 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t230);
        let t234 = t233 * t30;
        let t239 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t234 * t79 + t153);
        let t241 = piecewise5(t15, 0.0, t11, 0.0, t8 - t211);
        let t244 = piecewise3(t89, 0.0, 5.0 / 3.0 * t91 * t241);
        let t245 = t244 * t30;
        let t249 = sigma2 * sigma2;
        let t250 = t157 * t249;
        let t251 = t96 * t96;
        let t252 = t251 * t96;
        let t254 = 1.0 / t97 / t252;
        let t259 = t96 * rho1;
        let t261 = 1.0 / t98 / t259;
        let t265 = 1.0 / rho1;
        let t268 = 4.0 / 3.0 * t117 * param_expo * t265;
        let t269 = t155 * t250 * t254 * t104 / 216.0 - t107 * t95 * t261 / 9.0 + t268;
        let t271 = t129 * t129;
        let t272 = 1.0 / t271;
        let t273 = t118 * t272;
        let t277 = 1.0 / t97 / t96 * param_a * t125;
        let t280 = t189 * sigma2;
        let t284 = 6.0 * t193 * t101 + 144.0;
        let t285 = f64::sqrt(t284);
        let t286 = 1.0 / t285;
        let t287 = param_b * t286;
        let t288 = t261 * param_a * t287;
        let t291 = -t119 * t277 / 9.0 - 2.0 / 3.0 * t280 * t288 - t268;
        let t293 = t269 * t130 - t273 * t291;
        let t298 = piecewise3(t84, 0.0, 3.0 / 20.0 * t6 * t245 * t132 + t224 + 3.0 / 20.0 * t6 * t94 * t293);
        let tvrho1 = t83 + t136 + t7 * (t239 + t298);
        vrho[ip * 2 + 1] += tvrho1;
        let t301 = t160 * rho0;
        let t303 = 1.0 / t40 / t301;
        let t304 = t157 * t303;
        let t305 = t47 * sigma0;
        let t312 = 1.0 / sigma0;
        let t315 = t63 * param_expo * t312 / 2.0;
        let t316 = -t155 * t304 * t305 / 576.0 + t50 * t37 * t43 / 24.0 - t315;
        let t319 = t55 / t56;
        let t323 = param_a * param_b;
        let t324 = t323 * t198;
        let t327 = t319 * t73 / 24.0 + t189 * t43 * t324 / 4.0 + t315;
        let t329 = -t182 * t327 + t316 * t77;
        let t333 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t329);
        let tvsigma0 = t7 * t333;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t334 = t251 * rho1;
        let t336 = 1.0 / t97 / t334;
        let t337 = t157 * t336;
        let t338 = t104 * sigma2;
        let t345 = 1.0 / sigma2;
        let t348 = t117 * param_expo * t345 / 2.0;
        let t349 = -t155 * t337 * t338 / 576.0 + t107 * t37 * t100 / 24.0 - t348;
        let t352 = t55 / t110;
        let t356 = t323 * t286;
        let t359 = t352 * t126 / 24.0 + t189 * t100 * t356 / 4.0 + t348;
        let t361 = t349 * t130 - t273 * t359;
        let t365 = piecewise3(t84, 0.0, 3.0 / 20.0 * t6 * t94 * t361);
        let tvsigma2 = t7 * t365;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
