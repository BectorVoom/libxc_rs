//! MGGA_X_TAU_HCTH exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tau_hcth.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_tau_hcth_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_cx_local_1: f64,
    param_cx_local_2: f64,
    param_cx_local_3: f64,
    param_cx_nlocal_1: f64,
    param_cx_nlocal_2: f64,
    param_cx_nlocal_3: f64,
    param_cx_nlocal_0: f64,
    param_cx_local_0: f64,
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
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
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
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t29 = param_cx_local_0;
        let t30 = param_cx_local_1;
        let t31 = t30 * sigma0;
        let t32 = rho0 * rho0;
        let t33 = pow_1_3(rho0);
        let t34 = t33 * t33;
        let t36 = 1.0 / t34 / t32;
        let t39 = 1.0 + 0.004 * sigma0 * t36;
        let t40 = 1.0 / t39;
        let t41 = t36 * t40;
        let t44 = param_cx_local_2;
        let t45 = sigma0 * sigma0;
        let t46 = t44 * t45;
        let t47 = t32 * t32;
        let t48 = t47 * rho0;
        let t50 = 1.0 / t33 / t48;
        let t51 = t39 * t39;
        let t52 = 1.0 / t51;
        let t53 = t50 * t52;
        let t56 = param_cx_local_3;
        let t57 = t45 * sigma0;
        let t58 = t56 * t57;
        let t59 = t47 * t47;
        let t60 = 1.0 / t59;
        let t61 = t51 * t39;
        let t62 = 1.0 / t61;
        let t63 = t60 * t62;
        let t66 = param_cx_nlocal_0;
        let t67 = param_cx_nlocal_1;
        let t68 = t67 * sigma0;
        let t71 = param_cx_nlocal_2;
        let t72 = t71 * t45;
        let t75 = param_cx_nlocal_3;
        let t76 = t75 * t57;
        let t79 = t66 + 0.004 * t68 * t41 + 1.6e-05 * t72 * t53 + 6.4e-08 * t76 * t63;
        let t80 = M_CBRT6;
        let t81 = t80 * t80;
        let t82 = M_PI * M_PI;
        let t83 = pow_1_3(t82);
        let t84 = t83 * t83;
        let t86 = 3.0 / 10.0 * t81 * t84;
        let t88 = 1.0 / t34 / rho0;
        let t89 = tau0 * t88;
        let t90 = t86 - t89;
        let t91 = t86 + t89;
        let t92 = 1.0 / t91;
        let t94 = t90 * t90;
        let t95 = t94 * t90;
        let t96 = t91 * t91;
        let t97 = t96 * t91;
        let t98 = 1.0 / t97;
        let t101 = t94 * t94;
        let t102 = t101 * t90;
        let t103 = t96 * t96;
        let t105 = 1.0 / t103 / t91;
        let t107 = t102 * t105 + t90 * t92 - 2.0 * t95 * t98;
        let t109 = t29 + 0.004 * t31 * t41 + 1.6e-05 * t46 * t53 + 6.4e-08 * t58 * t63 + t79 * t107;
        let t113 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t109);
        let t114 = rho1 <= dens_threshold;
        let t115 = -t17;
        let t117 = piecewise5(t15, t12, t11, t16, t115 * t8);
        let t118 = 1.0 + t117;
        let t119 = t118 <= zeta_threshold;
        let t120 = pow_1_3(t118);
        let t122 = piecewise3(t119, t23, t120 * t118);
        let t123 = t122 * t27;
        let t124 = t30 * sigma2;
        let t125 = rho1 * rho1;
        let t126 = pow_1_3(rho1);
        let t127 = t126 * t126;
        let t129 = 1.0 / t127 / t125;
        let t132 = 1.0 + 0.004 * sigma2 * t129;
        let t133 = 1.0 / t132;
        let t134 = t129 * t133;
        let t137 = sigma2 * sigma2;
        let t138 = t44 * t137;
        let t139 = t125 * t125;
        let t140 = t139 * rho1;
        let t142 = 1.0 / t126 / t140;
        let t143 = t132 * t132;
        let t144 = 1.0 / t143;
        let t145 = t142 * t144;
        let t148 = t137 * sigma2;
        let t149 = t56 * t148;
        let t150 = t139 * t139;
        let t151 = 1.0 / t150;
        let t152 = t143 * t132;
        let t153 = 1.0 / t152;
        let t154 = t151 * t153;
        let t157 = t67 * sigma2;
        let t160 = t71 * t137;
        let t163 = t75 * t148;
        let t166 = t66 + 0.004 * t157 * t134 + 1.6e-05 * t160 * t145 + 6.4e-08 * t163 * t154;
        let t168 = 1.0 / t127 / rho1;
        let t169 = tau1 * t168;
        let t170 = t86 - t169;
        let t171 = t86 + t169;
        let t172 = 1.0 / t171;
        let t174 = t170 * t170;
        let t175 = t174 * t170;
        let t176 = t171 * t171;
        let t177 = t176 * t171;
        let t178 = 1.0 / t177;
        let t181 = t174 * t174;
        let t182 = t181 * t170;
        let t183 = t176 * t176;
        let t185 = 1.0 / t183 / t171;
        let t187 = t170 * t172 - 2.0 * t175 * t178 + t182 * t185;
        let t189 = t29 + 0.004 * t124 * t134 + 1.6e-05 * t138 * t145 + 6.4e-08 * t149 * t154 + t166 * t187;
        let t193 = piecewise3(t114, 0.0, -3.0 / 8.0 * t6 * t123 * t189);
        let tzk0 = t113 + t193;
        zk[ip] += tzk0;
    }
}
