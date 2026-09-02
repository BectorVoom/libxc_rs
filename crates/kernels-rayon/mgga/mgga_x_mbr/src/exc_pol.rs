//! MGGA_X_MBR exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mbr.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::br89::{xc_mgga_x_br89_get_x};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mbr_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_beta: f64,
    param_gamma: f64,
    param_lambda: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t7 = 2.0 * rho0 * t4 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t11 = 2.0 * rho1 * t4 <= zeta_threshold;
        let t12 = -t8;
        let t13 = rho0 - rho1;
        let t15 = piecewise5(t7, t8, t11, t12, t13 * t4);
        let t16 = 1.0 + t15;
        let t17 = t16 <= zeta_threshold;
        let t18 = pow_1_3(zeta_threshold);
        let t19 = t18 * zeta_threshold;
        let t20 = pow_1_3(t16);
        let t22 = piecewise3(t17, t19, t20 * t16);
        let t23 = pow_1_3(t3);
        let t24 = t22 * t23;
        let t26 = pow_1_3(1.0 / M_PI);
        let t27 = 1.0 / t26;
        let t28 = t24 * t27;
        let t29 = M_CBRT4;
        let t30 = param_lambda * param_lambda;
        let t31 = t30 - param_lambda + 1.0 / 2.0;
        let t32 = pow_1_3(rho0);
        let t33 = t32 * t32;
        let t35 = 1.0 / t33 / rho0;
        let t37 = 2.0 * tau0 * t35;
        let t38 = M_CBRT6;
        let t39 = t38 * t38;
        let t40 = M_PI * M_PI;
        let t41 = pow_1_3(t40);
        let t42 = t41 * t41;
        let t43 = t39 * t42;
        let t44 = 3.0 / 5.0 * t43;
        let t45 = rho0 * rho0;
        let t47 = 1.0 / t33 / t45;
        let t54 = pow_2(2.0 * param_lambda - 1.0);
        let t55 = t54 * t38;
        let t56 = 1.0 / t42;
        let t57 = t56 * sigma0;
        let t61 = t54 * t54;
        let t62 = param_beta * t61;
        let t63 = t62 * t39;
        let t65 = 1.0 / t41 / t40;
        let t66 = sigma0 * sigma0;
        let t67 = t65 * t66;
        let t68 = t45 * t45;
        let t69 = t68 * rho0;
        let t71 = 1.0 / t32 / t69;
        let t75 = 1.0 + 175.0 / 162.0 * t55 * t57 * t47 + t63 * t67 * t71 / 576.0;
        let t76 = rmath::pow(t75, 1.0 / 5.0);
        let t80 = t54 * sigma0;
        let t86 = -t31 * (t37 - t44 - sigma0 * t47 / 36.0) - t43 * (t76 - 1.0) / 5.0 + param_gamma * (t37 - t80 * t47 / 4.0) / 3.0;
        let t87 = rmath::abs(t86);
        let t88 = t87 < 5e-13;
        let t89 = -t86;
        let t90 = 0.0 < t89;
        let t91 = piecewise3(t90, 5e-13, -5e-13);
        let t92 = piecewise3(t88, t91, t89);
        let t93 = xc_mgga_x_br89_get_x(t92);
        let t95 = rmath::exp(t93 / 3.0);
        let t96 = t29 * t95;
        let t97 = rmath::exp(-t93);
        let t99 = 1.0 + t93 / 2.0;
        let t100 = t97 * t99;
        let t101 = 1.0 - t100;
        let t102 = 1.0 / t93;
        let t103 = t101 * t102;
        let t104 = t96 * t103;
        let t107 = piecewise3(t2, 0.0, -t28 * t104 / 4.0);
        let t108 = rho1 <= dens_threshold;
        let t109 = -t13;
        let t111 = piecewise5(t11, t8, t7, t12, t109 * t4);
        let t112 = 1.0 + t111;
        let t113 = t112 <= zeta_threshold;
        let t114 = pow_1_3(t112);
        let t116 = piecewise3(t113, t19, t114 * t112);
        let t117 = t116 * t23;
        let t118 = t117 * t27;
        let t119 = pow_1_3(rho1);
        let t120 = t119 * t119;
        let t122 = 1.0 / t120 / rho1;
        let t124 = 2.0 * tau1 * t122;
        let t125 = rho1 * rho1;
        let t127 = 1.0 / t120 / t125;
        let t132 = t56 * sigma2;
        let t136 = sigma2 * sigma2;
        let t137 = t65 * t136;
        let t138 = t125 * t125;
        let t139 = t138 * rho1;
        let t141 = 1.0 / t119 / t139;
        let t145 = 1.0 + 175.0 / 162.0 * t55 * t132 * t127 + t63 * t137 * t141 / 576.0;
        let t146 = rmath::pow(t145, 1.0 / 5.0);
        let t150 = t54 * sigma2;
        let t156 = -t31 * (t124 - t44 - sigma2 * t127 / 36.0) - t43 * (t146 - 1.0) / 5.0 + param_gamma * (t124 - t150 * t127 / 4.0) / 3.0;
        let t157 = rmath::abs(t156);
        let t158 = t157 < 5e-13;
        let t159 = -t156;
        let t160 = 0.0 < t159;
        let t161 = piecewise3(t160, 5e-13, -5e-13);
        let t162 = piecewise3(t158, t161, t159);
        let t163 = xc_mgga_x_br89_get_x(t162);
        let t165 = rmath::exp(t163 / 3.0);
        let t166 = t29 * t165;
        let t167 = rmath::exp(-t163);
        let t169 = 1.0 + t163 / 2.0;
        let t170 = t167 * t169;
        let t171 = 1.0 - t170;
        let t172 = 1.0 / t163;
        let t173 = t171 * t172;
        let t174 = t166 * t173;
        let t177 = piecewise3(t108, 0.0, -t118 * t174 / 4.0);
        let tzk0 = t107 + t177;
        zk[ip] += tzk0;
    }
}
