//! LDA_K_ZLP vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 33 shared lines across all orders.
//! Delta: 24 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_K_ZLP vxc -- polarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_k_zlp_vxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        // --- shared preamble (33 lines) ---
        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = 1.0 / t4;
        let t7 = M_CBRT4;
        let t8 = t2 * t5 * t7;
        let t9 = rho0 - rho1;
        let t10 = rho0 + rho1;
        let t11 = 1.0 / t10;
        let t12 = t9 * t11;
        let t13 = 1.0 + t12;
        let t14 = t13 <= zeta_threshold;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * t15;
        let t17 = t16 * zeta_threshold;
        let t18 = pow_1_3(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3(t14, t17, t19 * t13);
        let t22 = 1.0 - t12;
        let t23 = t22 <= zeta_threshold;
        let t24 = pow_1_3(t22);
        let t25 = t24 * t24;
        let t27 = piecewise3(t23, t17, t25 * t22);
        let t29 = t21 / 2.0 + t27 / 2.0;
        let t30 = pow_1_3(t10);
        let t31 = t30 * t30;
        let t32 = t29 * t31;
        let t33 = 1.0 / t30;
        let t35 = 1.0 + 510.2040816326531 * t33;
        let t36 = f64::ln(t35);
        let t39 = 1.0 - 0.00196 * t30 * t36;
        let t41 = t8 * t32 * t39;
        let tzk0 = 1.0790666666666666 * t41;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (24 lines) ---
        let t42 = 1.7984444444444445 * t41;
        let t43 = t31 * t10;
        let t45 = t43 * t2 * t5;
        let t46 = t10 * t10;
        let t47 = 1.0 / t46;
        let t48 = t9 * t47;
        let t49 = t11 - t48;
        let t52 = piecewise3(t14, 0.0, 5.0 / 3.0 * t19 * t49);
        let t53 = -t49;
        let t56 = piecewise3(t23, 0.0, 5.0 / 3.0 * t25 * t53);
        let t58 = t52 / 2.0 + t56 / 2.0;
        let t59 = t7 * t58;
        let t63 = t7 * t29;
        let t67 = 1.0 / t35;
        let t70 = -0.0006533333333333333 / t31 * t36 + 0.3333333333333333 * t11 * t67;
        let t73 = 1.0790666666666666 * t45 * t63 * t70;
        let tvrho0 = t42 + 1.0790666666666666 * t45 * t59 * t39 + t73;
        vrho[ip * 2] += tvrho0;
        let t74 = -t11 - t48;
        let t77 = piecewise3(t14, 0.0, 5.0 / 3.0 * t19 * t74);
        let t78 = -t74;
        let t81 = piecewise3(t23, 0.0, 5.0 / 3.0 * t25 * t78);
        let t84 = t7 * (t77 / 2.0 + t81 / 2.0);
        let t85 = t84 * t39;
        let tvrho1 = t42 + 1.0790666666666666 * t45 * t85 + t73;
        vrho[ip * 2 + 1] += tvrho1;
    }
}
