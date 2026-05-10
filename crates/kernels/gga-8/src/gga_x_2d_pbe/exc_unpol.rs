//! GGA_X_2D_PBE exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 18 shared lines across all orders.
//! Delta: 18 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_2d_pbe_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (18 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = f64::sqrt(M_PI);
        let t5 = 1.0 <= zeta_threshold;
        let t6 = zeta_threshold - 1.0;
        let t8 = piecewise5(t5, t6, t5, -t6, 0.0);
        let t9 = 1.0 + t8;
        let t11 = f64::sqrt(zeta_threshold);
        let t13 = f64::sqrt(t9);
        let t15 = piecewise3(t9 <= zeta_threshold, t11 * zeta_threshold, t13 * t9);
        let t16 = 1.0 / t3 * t15;
        let t17 = M_SQRT2;
        let t18 = f64::sqrt(rho[ip]);
        let t20 = rho[ip] * rho[ip];
        let t21 = t20 * rho[ip];
        let t25 = 0.4604e0 + 0.14106971928508582281e-1 * sigma[ip] / t21;
        let t28 = 0.14604e1 - 0.21196816e0 / t25;
        let t32 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t17 * t18 * t28);
        let tzk0 = 2.0 * t32;
        zk[ip] += tzk0;
    }
}
