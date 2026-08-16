//! MGGA_X_GVT4 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gvt4.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_gvt4_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
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
        let t3 = M_CBRTPI;
        let t4 = 1.0 / t3;
        let t5 = rho0 + rho1;
        let t6 = 1.0 / t5;
        let t9 = 2.0 * rho0 * t6 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t13 = 2.0 * rho1 * t6 <= zeta_threshold;
        let t14 = -t10;
        let t15 = rho0 - rho1;
        let t17 = piecewise5(t9, t10, t13, t14, t15 * t6);
        let t18 = 1.0 + t17;
        let t19 = t18 <= zeta_threshold;
        let t20 = pow_1_3(zeta_threshold);
        let t21 = t20 * zeta_threshold;
        let t22 = pow_1_3(t18);
        let t24 = piecewise3(t19, t21, t22 * t18);
        let t25 = t4 * t24;
        let t26 = pow_1_3(t5);
        let t27 = t25 * t26;
        let t28 = rho0 * rho0;
        let t29 = pow_1_3(rho0);
        let t30 = t29 * t29;
        let t32 = 1.0 / t30 / t28;
        let t33 = sigma0 * t32;
        let t36 = 1.0 / t30 / rho0;
        let t37 = tau0 * t36;
        let t39 = M_CBRT6;
        let t40 = t39 * t39;
        let t41 = M_PI * M_PI;
        let t42 = pow_1_3(t41);
        let t43 = t42 * t42;
        let t44 = t40 * t43;
        let t45 = 0.1120356e-2 * t44;
        let t46 = 1.0 + 0.186726e-2 * t33 + 0.373452e-2 * t37 - t45;
        let t51 = 0.37501956e-2 * t44;
        let t52 = -0.3556788e-2 * t33 + 0.12500652e-1 * t37 - t51;
        let t53 = t46 * t46;
        let t54 = 1.0 / t53;
        let t56 = sigma0 * sigma0;
        let t57 = t28 * t28;
        let t58 = t57 * rho0;
        let t60 = 1.0 / t29 / t58;
        let t64 = 3.0 / 5.0 * t44;
        let t65 = 2.0 * t37 - t64;
        let t68 = t65 * t65;
        let t70 = -0.2354518e-4 * t56 * t60 - 0.1282732e-3 * t33 * t65 + 0.3574822e-3 * t68;
        let t71 = t53 * t46;
        let t72 = 1.0 / t71;
        let t76 = pow_1_3(1.0 / M_PI);
        let t77 = 1.0 / t76;
        let t79 = M_CBRT4;
        let t80 = (-0.9800683e0 / t46 + t52 * t54 + t70 * t72) * t77 * t79;
        let t83 = piecewise3(t2, 0.0, t27 * t80 / 4.0);
        let t84 = rho1 <= dens_threshold;
        let t85 = -t15;
        let t87 = piecewise5(t13, t10, t9, t14, t85 * t6);
        let t88 = 1.0 + t87;
        let t89 = t88 <= zeta_threshold;
        let t90 = pow_1_3(t88);
        let t92 = piecewise3(t89, t21, t90 * t88);
        let t93 = t4 * t92;
        let t94 = t93 * t26;
        let t95 = rho1 * rho1;
        let t96 = pow_1_3(rho1);
        let t97 = t96 * t96;
        let t99 = 1.0 / t97 / t95;
        let t100 = sigma2 * t99;
        let t103 = 1.0 / t97 / rho1;
        let t104 = tau1 * t103;
        let t106 = 1.0 + 0.186726e-2 * t100 + 0.373452e-2 * t104 - t45;
        let t111 = -0.3556788e-2 * t100 + 0.12500652e-1 * t104 - t51;
        let t112 = t106 * t106;
        let t113 = 1.0 / t112;
        let t115 = sigma2 * sigma2;
        let t116 = t95 * t95;
        let t117 = t116 * rho1;
        let t119 = 1.0 / t96 / t117;
        let t123 = 2.0 * t104 - t64;
        let t126 = t123 * t123;
        let t128 = -0.2354518e-4 * t115 * t119 - 0.1282732e-3 * t100 * t123 + 0.3574822e-3 * t126;
        let t129 = t112 * t106;
        let t130 = 1.0 / t129;
        let t134 = (-0.9800683e0 / t106 + t111 * t113 + t128 * t130) * t77 * t79;
        let t137 = piecewise3(t84, 0.0, t94 * t134 / 4.0);
        let tzk0 = t83 + t137;
        zk[ip] += tzk0;
    }
}
