//! MGGA_XC_LP90 vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 10 shared lines across all orders.
//! Delta: 10 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_xc_lp90_vxc_unpol(
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
        // --- shared preamble (10 lines) ---
        let t2 = rho[ip] * rho[ip];
        let t3 = pow_1_3(rho[ip]);
        let t4 = t3 * t3;
        let t6 = 1.0 / t4 / t2;
        let t10 = 1.0 / t4 / rho[ip];
        let t13 = 0.80569e0 + 0.37655e-3 * sigma[ip] * t6 - 0.37655e-3 * lapl[ip] * t10;
        let t14 = 1.0 / t3;
        let t15 = t14 + 0.40743e-2;
        let t16 = 1.0 / t15;
        let tzk0 = -t13 * t16;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (10 lines) ---
        let t18 = t2 * rho[ip];
        let t20 = 1.0 / t4 / t18;
        let t25 = -0.10041333333333333333e-2 * sigma[ip] * t20 + 0.62758333333333333333e-3 * lapl[ip] * t6;
        let t29 = t15 * t15;
        let t30 = 1.0 / t29;
        let tvrho0 = tzk0 - rho[ip] * t25 * t16 - t14 * t13 * t30 / 3.0;
        vrho[ip] += tvrho0;
        let t33 = t10 * t16;
        let tvsigma0 = -0.37655e-3 * t33;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.37655e-3 / t4 * t16;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
    }
}
