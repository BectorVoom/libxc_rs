//! GGA_K_RATIONAL_P vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 30 shared lines across all orders.
//! Delta: 12 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_k_rational_p_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_C2: f64,
    param_p: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (30 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t24 = 1.0 / param_p;
        let t26 = M_CBRT6;
        let t28 = M_PI * M_PI;
        let t29 = pow_1_3(t28);
        let t30 = t29 * t29;
        let t31 = 1.0 / t30;
        let t32 = t31 * sigma[ip];
        let t33 = M_CBRT2;
        let t34 = t33 * t33;
        let t35 = rho[ip] * rho[ip];
        let t42 = 1.0 + param_C2 * t24 * t26 * t32 * t34 / t22 / t35 / 24.0;
        let t43 = f64::powf(t42, -param_p);
        let t47 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t20 * t22 * t43);
        let tzk0 = 2.0 * t47;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (12 lines) ---
        let t53 = t35 * rho[ip];
        let t57 = t7 * t20 / t53 * t43;
        let t58 = param_C2 * t26;
        let t60 = sigma[ip] * t34;
        let t61 = 1.0 / t42;
        let t63 = t58 * t31 * t60 * t61;
        let t67 = piecewise3(t2, 0.0, t7 * t20 / t21 * t43 / 10.0 + t57 * t63 / 60.0);
        let tvrho0 = 2.0 * rho[ip] * t67 + 2.0 * t47;
        vrho[ip] += tvrho0;
        let t74 = t31 * t34;
        let t76 = t58 * t74 * t61;
        let t79 = piecewise3(t2, 0.0, -t7 * t20 / t35 * t43 * t76 / 160.0);
        let tvsigma0 = 2.0 * rho[ip] * t79;
        vsigma[ip] += tvsigma0;
    }
}
