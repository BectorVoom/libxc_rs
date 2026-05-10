//! LDA_C_2D_AMGB vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 21 shared lines across all orders.
//! Delta: 18 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_2D_AMGB vxc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_c_2d_amgb_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
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
        // --- vxc delta (this level) (18 lines) ---
        let t38 = rho[ip] * rho[ip];
        let t39 = 1.0 / t38;
        let t42 = 1.0 / t1 / t38;
        let t44 = -0.02434861701925381 * t7 - 0.018219548589342285 * t39 - 0.000905920503043323 * t42;
        let t45 = t44 * t21;
        let t46 = t18 * t18;
        let t47 = 1.0 / t46;
        let t48 = t9 * t47;
        let t50 = f64::sqrt(t13);
        let t51 = t50 * t12;
        let t56 = -0.2827154003157807 * t7 + 0.0155175 * t51 * t7 - 0.10821581200590331 * t39 - 0.00470608053528999 * t42;
        let t57 = 1.0 / t20;
        let t58 = t56 * t57;
        let t59 = t48 * t58;
        let t61 = t26 * t33;
        let t62 = t4 * t24 * t61;
        let t65 = t27 * t13 * t33;
        let tvrho0 = -0.1925 + t22 - t36 + rho[ip] * (t45 - t59 - 0.2840597424304148 * t62 - 2.0 / 3.0 * t65);
        vrho[ip] += tvrho0;
    }
}
