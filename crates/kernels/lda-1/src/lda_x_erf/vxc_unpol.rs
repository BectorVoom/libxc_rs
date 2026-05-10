//! LDA_X_ERF vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 47 shared lines across all orders.
//! Delta: 25 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::erf::{erf_approx};

/// LDA_X_ERF vxc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_x_erf_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (47 lines) ---
        let t1 = M_CBRT3;
        let t3 = pow_1_3(1.0 / M_PI);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = t1 * t3 * t6;
        let t8 = M_CBRT2;
        let t9 = t8 * t8;
        let t10 = 1.0 <= zeta_threshold;
        let t11 = pow_1_3(zeta_threshold);
        let t13 = piecewise3(t10, t11 * zeta_threshold, 1.0);
        let t14 = t9 * t13;
        let t15 = pow_1_3(rho[ip]);
        let t16 = pow_1_3(9.0);
        let t17 = t16 * t16;
        let t18 = t3 * t3;
        let t20 = t17 * t18 * param_hyb_omega_0;
        let t23 = piecewise3(t10, t11, 1.0);
        let t24 = 1.0 / t23;
        let t27 = t20 * t1 / t15 * t24 / 18.0;
        let t28 = 1.35 <= t27;
        let t29 = 1.35 < t27;
        let t30 = piecewise3(t29, t27, 1.35);
        let t31 = t30 * t30;
        let t34 = t31 * t31;
        let t35 = 1.0 / t34;
        let t37 = t34 * t31;
        let t38 = 1.0 / t37;
        let t40 = t34 * t34;
        let t41 = 1.0 / t40;
        let t44 = 1.0 / t40 / t31;
        let t47 = 1.0 / t40 / t34;
        let t50 = 1.0 / t40 / t37;
        let t52 = t40 * t40;
        let t53 = 1.0 / t52;
        let t56 = piecewise3(t29, 1.35, t27);
        let t57 = f64::sqrt(M_PI);
        let t58 = 1.0 / t56;
        let t60 = erf_approx(t58 / 2.0);
        let t62 = t56 * t56;
        let t63 = 1.0 / t62;
        let t65 = f64::exp(-t63 / 4.0);
        let t66 = t65 - 1.0;
        let t69 = t65 - 3.0 / 2.0 - 2.0 * t62 * t66;
        let t72 = 2.0 * t56 * t69 + t57 * t60;
        let t76 = piecewise3(t28, 1.0 / t31 / 36.0 - t35 / 960.0 + t38 / 26880.0 - t41 / 829440.0 + t44 / 28385280.0 - t47 / 1073479680.0 + t50 / 44590694400.0 - t53 / 2021444812800.0, 1.0 - 8.0 / 3.0 * t56 * t72);
        let t79 = t7 * t14 * t15 * t76;
        let tzk0 = -3.0 / 16.0 * t79;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (25 lines) ---
        let t82 = t15 * rho[ip];
        let t84 = t82 * t1 * t3;
        let t85 = t6 * t9;
        let t86 = t31 * t30;
        let t87 = 1.0 / t86;
        let t92 = t20 * t1 / t82 * t24 / 54.0;
        let t93 = piecewise3(t29, -t92, 0.0);
        let t96 = t34 * t30;
        let t97 = 1.0 / t96;
        let t100 = t34 * t86;
        let t101 = 1.0 / t100;
        let t105 = 1.0 / t40 / t30;
        let t109 = 1.0 / t40 / t86;
        let t113 = 1.0 / t40 / t96;
        let t117 = 1.0 / t40 / t100;
        let t121 = 1.0 / t52 / t30;
        let t125 = piecewise3(t29, 0.0, -t92);
        let t127 = t65 * t63;
        let t131 = t62 * t56;
        let t132 = 1.0 / t131;
        let t136 = t56 * t66;
        let t141 = t132 * t125 * t65 / 2.0 - 4.0 * t136 * t125 - t58 * t125 * t65;
        let t144 = -t127 * t125 + 2.0 * t125 * t69 + 2.0 * t56 * t141;
        let t148 = piecewise3(t28, -t87 * t93 / 18.0 + t97 * t93 / 240.0 - t101 * t93 / 4480.0 + t105 * t93 / 103680.0 - t109 * t93 / 2838528.0 + t113 * t93 / 89456640.0 - t117 * t93 / 3185049600.0 + t121 * t93 / 126340300800.0, -8.0 / 3.0 * t125 * t72 - 8.0 / 3.0 * t56 * t144);
        let tvrho0 = -t79 / 4.0 - 3.0 / 16.0 * t84 * t85 * t13 * t148;
        vrho[ip] += tvrho0;
    }
}
