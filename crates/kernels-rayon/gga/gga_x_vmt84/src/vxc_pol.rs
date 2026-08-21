//! GGA_X_VMT84 vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_vmt84.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_vmt84_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_mu: f64,
    param_alpha: f64,
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
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = param_mu * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t38 = t37 * t35;
        let t39 = 1.0 / t38;
        let t41 = param_alpha * t28;
        let t42 = t33 * sigma0;
        let t43 = t42 * t39;
        let t46 = rmath::exp(-t41 * t43 / 24.0);
        let t49 = 1.0 + t29 * t43 / 24.0;
        let t50 = 1.0 / t49;
        let t51 = t46 * t50;
        let t55 = t28 * t28;
        let t56 = param_alpha * t55;
        let t58 = 1.0 / t31 / t30;
        let t59 = sigma0 * sigma0;
        let t60 = t58 * t59;
        let t61 = t35 * t35;
        let t62 = t61 * rho0;
        let t64 = 1.0 / t36 / t62;
        let t68 = rmath::exp(-t56 * t60 * t64 / 576.0);
        let t70 = (1.0 - t68) * t55;
        let t71 = 1.0 / sigma0;
        let t72 = t32 * t71;
        let t76 = t34 * sigma0 * t39 * t51 / 24.0 + 4.0 * t70 * t72 * t38 + t68;
        let t80 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t76);
        let t81 = rho1 <= dens_threshold;
        let t82 = -t16;
        let t84 = piecewise5(t14, t11, t10, t15, t82 * t7);
        let t85 = 1.0 + t84;
        let t86 = t85 <= zeta_threshold;
        let t87 = pow_1_3(t85);
        let t89 = piecewise3(t86, t22, t87 * t85);
        let t90 = t89 * t26;
        let t91 = rho1 * rho1;
        let t92 = pow_1_3(rho1);
        let t93 = t92 * t92;
        let t94 = t93 * t91;
        let t95 = 1.0 / t94;
        let t97 = t33 * sigma2;
        let t98 = t97 * t95;
        let t101 = rmath::exp(-t41 * t98 / 24.0);
        let t104 = 1.0 + t29 * t98 / 24.0;
        let t105 = 1.0 / t104;
        let t106 = t101 * t105;
        let t110 = sigma2 * sigma2;
        let t111 = t58 * t110;
        let t112 = t91 * t91;
        let t113 = t112 * rho1;
        let t115 = 1.0 / t92 / t113;
        let t119 = rmath::exp(-t56 * t111 * t115 / 576.0);
        let t121 = (1.0 - t119) * t55;
        let t122 = 1.0 / sigma2;
        let t123 = t32 * t122;
        let t127 = t34 * sigma2 * t95 * t106 / 24.0 + 4.0 * t121 * t123 * t94 + t119;
        let t131 = piecewise3(t81, 0.0, -3.0 / 8.0 * t5 * t90 * t127);
        let tzk0 = t80 + t131;
        zk[ip] += tzk0;
        let t132 = t6 * t6;
        let t133 = 1.0 / t132;
        let t134 = t16 * t133;
        let t136 = piecewise5(t10, 0.0, t14, 0.0, t7 - t134);
        let t139 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t136);
        let t140 = t139 * t26;
        let t144 = t26 * t26;
        let t145 = 1.0 / t144;
        let t146 = t25 * t145;
        let t149 = t5 * t146 * t76 / 8.0;
        let t150 = t35 * rho0;
        let t152 = 1.0 / t37 / t150;
        let t153 = sigma0 * t152;
        let t157 = param_mu * t55;
        let t158 = t157 * t60;
        let t159 = t61 * t35;
        let t161 = 1.0 / t36 / t159;
        let t163 = t161 * param_alpha * t51;
        let t166 = param_mu * param_mu;
        let t168 = t166 * t55 * t58;
        let t169 = t59 * t161;
        let t170 = t49 * t49;
        let t171 = 1.0 / t170;
        let t172 = t46 * t171;
        let t176 = t41 * t33;
        let t180 = t37 * rho0;
        let t184 = t56 * t58;
        let t188 = -t34 * t153 * t51 / 9.0 + t158 * t163 / 216.0 + t168 * t169 * t172 / 216.0 - 2.0 / 9.0 * t176 * t153 * t68 + 32.0 / 3.0 * t70 * t72 * t180 + t184 * t169 * t68 / 108.0;
        let t193 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t140 * t76 - t149 - 3.0 / 8.0 * t5 * t27 * t188);
        let t194 = t82 * t133;
        let t196 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t194);
        let t199 = piecewise3(t86, 0.0, 4.0 / 3.0 * t87 * t196);
        let t200 = t199 * t26;
        let t204 = t89 * t145;
        let t207 = t5 * t204 * t127 / 8.0;
        let t209 = piecewise3(t81, 0.0, -3.0 / 8.0 * t5 * t200 * t127 - t207);
        let tvrho0 = t80 + t131 + t6 * (t193 + t209);
        vrho[ip * 2] += tvrho0;
        let t213 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t134);
        let t216 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t213);
        let t217 = t216 * t26;
        let t222 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t217 * t76 - t149);
        let t224 = piecewise5(t14, 0.0, t10, 0.0, t7 - t194);
        let t227 = piecewise3(t86, 0.0, 4.0 / 3.0 * t87 * t224);
        let t228 = t227 * t26;
        let t232 = t91 * rho1;
        let t234 = 1.0 / t93 / t232;
        let t235 = sigma2 * t234;
        let t239 = t157 * t111;
        let t240 = t112 * t91;
        let t242 = 1.0 / t92 / t240;
        let t244 = t242 * param_alpha * t106;
        let t247 = t110 * t242;
        let t248 = t104 * t104;
        let t249 = 1.0 / t248;
        let t250 = t101 * t249;
        let t257 = t93 * rho1;
        let t264 = -t34 * t235 * t106 / 9.0 + t239 * t244 / 216.0 + t168 * t247 * t250 / 216.0 - 2.0 / 9.0 * t176 * t235 * t119 + 32.0 / 3.0 * t121 * t123 * t257 + t184 * t247 * t119 / 108.0;
        let t269 = piecewise3(t81, 0.0, -3.0 / 8.0 * t5 * t228 * t127 - t207 - 3.0 / 8.0 * t5 * t90 * t264);
        let tvrho1 = t80 + t131 + t6 * (t222 + t269);
        vrho[ip * 2 + 1] += tvrho1;
        let t279 = t64 * param_alpha * t51;
        let t282 = sigma0 * t64;
        let t290 = 1.0 / t59;
        let t291 = t32 * t290;
        let t298 = t34 * t39 * t46 * t50 / 24.0 - t157 * t58 * sigma0 * t279 / 576.0 - t168 * t282 * t172 / 576.0 + t41 * t33 * t39 * t68 / 12.0 - 4.0 * t70 * t291 * t38 - t184 * t282 * t68 / 288.0;
        let t302 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t298);
        let tvsigma0 = t6 * t302;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t310 = t115 * param_alpha * t106;
        let t313 = sigma2 * t115;
        let t321 = 1.0 / t110;
        let t322 = t32 * t321;
        let t329 = t34 * t95 * t101 * t105 / 24.0 - t157 * t58 * sigma2 * t310 / 576.0 - t168 * t313 * t250 / 576.0 + t41 * t33 * t95 * t119 / 12.0 - 4.0 * t121 * t322 * t94 - t184 * t313 * t119 / 288.0;
        let t333 = piecewise3(t81, 0.0, -3.0 / 8.0 * t5 * t90 * t329);
        let tvsigma2 = t6 * t333;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
