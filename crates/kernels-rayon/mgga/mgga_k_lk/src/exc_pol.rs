//! MGGA_K_LK exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_lk.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_k_lk_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_kappa: f64,
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
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = rho0 + rho1;
        let t9 = 1.0 / t8;
        let t12 = 2.0 * rho0 * t9 <= zeta_threshold;
        let t13 = zeta_threshold - 1.0;
        let t16 = 2.0 * rho1 * t9 <= zeta_threshold;
        let t17 = -t13;
        let t18 = rho0 - rho1;
        let t20 = piecewise5(t12, t13, t16, t17, t18 * t9);
        let t21 = 1.0 + t20;
        let t22 = t21 <= zeta_threshold;
        let t23 = pow_1_3(zeta_threshold);
        let t24 = t23 * t23;
        let t25 = t24 * zeta_threshold;
        let t26 = pow_1_3(t21);
        let t27 = t26 * t26;
        let t29 = piecewise3(t22, t25, t27 * t21);
        let t30 = pow_1_3(t8);
        let t31 = t30 * t30;
        let t33 = M_CBRT6;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = 1.0 / t36;
        let t38 = t33 * t37;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t44 = sigma0 * t43;
        let t47 = t33 * t33;
        let t49 = 1.0 / t35 / t34;
        let t50 = t47 * t49;
        let t51 = lapl0 * lapl0;
        let t52 = t39 * rho0;
        let t54 = 1.0 / t40 / t52;
        let t57 = t50 * t51 * t54 / 5832.0;
        let t58 = t39 * t39;
        let t60 = 1.0 / t40 / t58;
        let t61 = sigma0 * t60;
        let t64 = t50 * t61 * lapl0 / 5184.0;
        let t65 = sigma0 * sigma0;
        let t66 = t58 * rho0;
        let t68 = 1.0 / t40 / t66;
        let t69 = t65 * t68;
        let t71 = t50 * t69 / 17496.0;
        let t72 = 1.0 / param_kappa;
        let t78 = 1.0 + (5.0 / 648.0 * t38 * t44 + t57 - t64 + t71 + 25.0 / 419904.0 * t50 * t69 * t72) * t72;
        let t80 = t38 * sigma0;
        let t81 = t57 - t64 + t71;
        let t83 = t43 * t81 * t72;
        let t86 = t34 * t34;
        let t87 = 1.0 / t86;
        let t88 = t65 * sigma0;
        let t89 = t87 * t88;
        let t90 = t58 * t58;
        let t91 = 1.0 / t90;
        let t92 = param_kappa * param_kappa;
        let t93 = 1.0 / t92;
        let t94 = t91 * t93;
        let t99 = 1.0 + (5.0 / 324.0 * t80 * t83 + 125.0 / 0.45349632e8 * t89 * t94) * t72;
        let t103 = 1.0 + param_kappa * (2.0 - 1.0 / t78 - 1.0 / t99);
        let t107 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t29 * t31 * t103);
        let t108 = rho1 <= dens_threshold;
        let t109 = -t18;
        let t111 = piecewise5(t16, t13, t12, t17, t109 * t9);
        let t112 = 1.0 + t111;
        let t113 = t112 <= zeta_threshold;
        let t114 = pow_1_3(t112);
        let t115 = t114 * t114;
        let t117 = piecewise3(t113, t25, t115 * t112);
        let t119 = rho1 * rho1;
        let t120 = pow_1_3(rho1);
        let t121 = t120 * t120;
        let t123 = 1.0 / t121 / t119;
        let t124 = sigma2 * t123;
        let t127 = lapl1 * lapl1;
        let t128 = t119 * rho1;
        let t130 = 1.0 / t120 / t128;
        let t133 = t50 * t127 * t130 / 5832.0;
        let t134 = t119 * t119;
        let t136 = 1.0 / t120 / t134;
        let t137 = sigma2 * t136;
        let t140 = t50 * t137 * lapl1 / 5184.0;
        let t141 = sigma2 * sigma2;
        let t142 = t134 * rho1;
        let t144 = 1.0 / t120 / t142;
        let t145 = t141 * t144;
        let t147 = t50 * t145 / 17496.0;
        let t153 = 1.0 + (5.0 / 648.0 * t38 * t124 + t133 - t140 + t147 + 25.0 / 419904.0 * t50 * t145 * t72) * t72;
        let t155 = t38 * sigma2;
        let t156 = t133 - t140 + t147;
        let t158 = t123 * t156 * t72;
        let t161 = t141 * sigma2;
        let t162 = t87 * t161;
        let t163 = t134 * t134;
        let t164 = 1.0 / t163;
        let t165 = t164 * t93;
        let t170 = 1.0 + (5.0 / 324.0 * t155 * t158 + 125.0 / 0.45349632e8 * t162 * t165) * t72;
        let t174 = 1.0 + param_kappa * (2.0 - 1.0 / t153 - 1.0 / t170);
        let t178 = piecewise3(t108, 0.0, 3.0 / 20.0 * t7 * t117 * t31 * t174);
        let tzk0 = t107 + t178;
        zk[ip] += tzk0;
    }
}
