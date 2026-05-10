//! LDA_XC_TETER93 exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 31 shared lines across all orders.
//! Delta: 31 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_XC_TETER93 exc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_xc_teter93_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (31 lines) ---
        let t2 = pow_1_3(zeta_threshold);
        let t4 = piecewise3(1.0 <= zeta_threshold, t2 * zeta_threshold, 1.0);
        let t7 = M_CBRT2;
        let t11 = (2.0 * t4 - 2.0) / (2.0 * t7 - 2.0);
        let t15 = M_CBRT3;
        let t16 = (2.217058676663745 + 0.6157402568883344 * t11) * t15;
        let t17 = 1.0 / M_PI;
        let t18 = pow_1_3(t17);
        let t19 = M_CBRT4;
        let t20 = t19 * t19;
        let t21 = t18 * t20;
        let t22 = pow_1_3(rho[ip]);
        let t23 = 1.0 / t22;
        let t29 = t15 * t15;
        let t30 = (0.7405551735357053 + 0.1574201515892867 * t11) * t29;
        let t31 = t18 * t18;
        let t32 = t31 * t19;
        let t33 = t22 * t22;
        let t35 = t32 / t33;
        let t40 = (0.01968227878617998 + 0.003532336663397157 * t11) * t17;
        let t41 = 1.0 / rho[ip];
        let t44 = 0.4581652932831429 + 0.119086804055547 * t11 + t16 * t21 * t23 / 4.0 + t30 * t35 / 4.0 + 3.0 / 4.0 * t40 * t41;
        let t45 = t15 * t18;
        let t51 = (4.504130959426697 + 0.2673612973836267 * t11) * t29;
        let t56 = (1.110667363742916 + 0.2052004607777787 * t11) * t17;
        let t61 = (0.02359291751427506 + 0.004200005045691381 * t11) * t15;
        let t63 = t18 * t17 * t20;
        let t65 = 1.0 / t22 / rho[ip];
        let t69 = 0.25 * t45 * t20 * t23 + t51 * t35 / 4.0 + 3.0 / 4.0 * t56 * t41 + 3.0 / 16.0 * t61 * t63 * t65;
        let t70 = 1.0 / t69;
        let tzk0 = -t44 * t70;
        zk[ip] += tzk0;
    }
}
