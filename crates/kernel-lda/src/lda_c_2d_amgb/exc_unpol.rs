//! LDA_C_2D_AMGB exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 21 shared lines across all orders.
//! Delta: 21 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_2D_AMGB exc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_2d_amgb_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (21 lines) ---
        let t1 = f64::sqrt(rho[ip]);
        let t2 = 1.0 / t1;
        let t4 = 1.0 / rho[ip];
        let t7 = 1.0 / t1 / rho[ip];
        let t9 = 0.04869723403850762 * t2 + 0.018219548589342285 * t4 + 0.000603947002028882 * t7;
        let t11 = f64::sqrt(M_PI);
        let t12 = 1.0 / t11;
        let t13 = t12 * t2;
        let t14 = pow_3_2(t13);
        let t18 = 0.5654308006315614 * t2 - 0.02069 * t14 + 0.10821581200590331 * t4 + 0.00313738702352666 * t7;
        let t20 = 1.0 + 1.0 / t18;
        let t21 = f64::ln(t20);
        let t22 = t9 * t21;
        let t24 = f64::exp(-0.7552241765370266 * t2);
        let t26 = M_SQRT2;
        let t27 = (t24 - 1.0) * t26;
        let t30 = f64::sqrt(zeta_threshold);
        let t32 = piecewise3(1.0 <= zeta_threshold, t30 * zeta_threshold, 1.0);
        let t33 = t32 - 1.0;
        let t36 = 4.0 / 3.0 * t27 * t12 * t1 * t33;
        let tzk0 = -0.1925 + t22 - t36;
        zk[ip] += tzk0;
    }
}
