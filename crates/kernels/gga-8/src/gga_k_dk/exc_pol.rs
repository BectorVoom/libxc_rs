//! GGA_K_DK exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 100 shared lines across all orders.
//! Delta: 100 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_k_dk_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_aa_0: f64,
    param_aa_1: f64,
    param_aa_2: f64,
    param_aa_3: f64,
    param_aa_4: f64,
    param_bb_0: f64,
    param_bb_1: f64,
    param_bb_2: f64,
    param_bb_3: f64,
    param_bb_4: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        // --- shared preamble (100 lines) ---
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
        let t29 = t6 * t28;
        let t30 = pow_1_3(t7);
        let t31 = t30 * t30;
        let t32 = param_aa_0;
        let t33 = param_aa_1;
        let t34 = t33 * sigma0;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t41 = param_aa_2;
        let t42 = sigma0 * sigma0;
        let t43 = t41 * t42;
        let t44 = t35 * t35;
        let t45 = t44 * rho0;
        let t47 = 1.0 / t36 / t45;
        let t49 = param_aa_3;
        let t50 = t42 * sigma0;
        let t51 = t49 * t50;
        let t52 = t44 * t44;
        let t53 = 1.0 / t52;
        let t55 = param_aa_4;
        let t56 = t42 * t42;
        let t57 = t55 * t56;
        let t58 = t52 * t35;
        let t60 = 1.0 / t37 / t58;
        let t62 = t34 * t39 + t43 * t47 + t51 * t53 + t57 * t60 + t32;
        let t63 = t31 * t62;
        let t64 = param_bb_0;
        let t65 = param_bb_1;
        let t66 = t65 * sigma0;
        let t68 = param_bb_2;
        let t69 = t68 * t42;
        let t71 = param_bb_3;
        let t72 = t71 * t50;
        let t74 = param_bb_4;
        let t75 = t74 * t56;
        let t77 = t66 * t39 + t69 * t47 + t72 * t53 + t75 * t60 + t64;
        let t78 = 1.0 / t77;
        let t79 = t63 * t78;
        let t82 = piecewise3(t1, 0.0, 3.0 / 20.0 * t29 * t79);
        let t83 = rho1 <= dens_threshold;
        let t84 = -t17;
        let t86 = piecewise5(t15, t12, t11, t16, t84 * t8);
        let t87 = 1.0 + t86;
        let t88 = t87 <= zeta_threshold;
        let t89 = pow_1_3(t87);
        let t90 = t89 * t89;
        let t92 = piecewise3(t88, t24, t90 * t87);
        let t93 = t6 * t92;
        let t94 = t33 * sigma2;
        let t95 = rho1 * rho1;
        let t96 = pow_1_3(rho1);
        let t97 = t96 * t96;
        let t99 = 1.0 / t97 / t95;
        let t101 = sigma2 * sigma2;
        let t102 = t41 * t101;
        let t103 = t95 * t95;
        let t104 = t103 * rho1;
        let t106 = 1.0 / t96 / t104;
        let t108 = t101 * sigma2;
        let t109 = t49 * t108;
        let t110 = t103 * t103;
        let t111 = 1.0 / t110;
        let t113 = t101 * t101;
        let t114 = t55 * t113;
        let t115 = t110 * t95;
        let t117 = 1.0 / t97 / t115;
        let t119 = t102 * t106 + t109 * t111 + t114 * t117 + t94 * t99 + t32;
        let t120 = t31 * t119;
        let t121 = t65 * sigma2;
        let t123 = t68 * t101;
        let t125 = t71 * t108;
        let t127 = t74 * t113;
        let t129 = t123 * t106 + t125 * t111 + t127 * t117 + t121 * t99 + t64;
        let t130 = 1.0 / t129;
        let t131 = t120 * t130;
        let t134 = piecewise3(t83, 0.0, 3.0 / 20.0 * t93 * t131);
        let tzk0 = t82 + t134;
        zk[ip] += tzk0;
    }
}
