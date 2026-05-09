//! MGGA_C_CS vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 21 shared lines across all orders.
//! Delta: 18 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_c_cs_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (21 lines) ---
        let t2 = pow_1_3(rho[ip]);
        let t3 = 1.0 / t2;
        let t5 = 1.0 + 0.34899999999999999998e0 * t3;
        let t6 = 1.0 / t5;
        let t8 = f64::exp(-0.2533e0 * t3);
        let t10 = zeta_threshold * zeta_threshold;
        let t11 = pow_1_3(zeta_threshold);
        let t12 = t11 * t11;
        let t14 = piecewise3(1.0 <= zeta_threshold, t12 * t10, 1.0);
        let t15 = M_CBRT2;
        let t16 = t14 * t15;
        let t17 = t15 * t15;
        let t18 = tau[ip] * t17;
        let t19 = t2 * t2;
        let t21 = 1.0 / t19 / rho[ip];
        let t23 = lapl[ip] * t17;
        let t29 = rho[ip] * rho[ip];
        let t31 = 1.0 / t19 / t29;
        let t36 = t16 * (t18 * t21 - t23 * t21 / 8.0) / 4.0 - sigma[ip] * t31 / 8.0 + lapl[ip] * t21 / 8.0;
        let t39 = 1.0 + 0.264e0 * t8 * t36;
        let tzk0 = -0.4918e-1 * t6 * t39;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (18 lines) ---
        let t42 = t5 * t5;
        let t43 = 1.0 / t42;
        let t44 = t3 * t43;
        let t47 = rho[ip] * t6;
        let t49 = 1.0 / t2 / rho[ip];
        let t50 = t49 * t8;
        let t60 = t29 * rho[ip];
        let t62 = 1.0 / t19 / t60;
        let t67 = t16 * (-5.0 / 3.0 * t18 * t31 + 5.0 / 24.0 * t23 * t31) / 4.0 + sigma[ip] * t62 / 3.0 - 5.0 / 24.0 * lapl[ip] * t31;
        let t70 = 0.222904e-1 * t50 * t36 + 0.264e0 * t8 * t67;
        let tvrho0 = tzk0 - 0.57212733333333333332e-2 * t44 * t39 - 0.4918e-1 * t47 * t70;
        vrho[ip] += tvrho0;
        let t73 = t21 * t6;
        let tvsigma0 = 0.162294e-2 * t73 * t8;
        vsigma[ip] += tvsigma0;
        let t78 = -t14 * t21 / 16.0 + t21 / 8.0;
        let t79 = t8 * t78;
        let tvlapl0 = -0.1298352e-1 * t47 * t79;
        vlapl[ip] += tvlapl0;
        let t84 = t8 * t14;
        let tvtau0 = -0.649176e-2 / t19 * t6 * t84;
        vtau[ip] += tvtau0;
    }
}
