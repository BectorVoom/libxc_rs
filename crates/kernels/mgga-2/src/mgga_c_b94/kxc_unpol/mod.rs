//! MGGA_C_B94 unpolarized kxc-level kernel.
//!
//! The Maple2c-translated body is split across 3 private internal modules
//! `part0`..`part2`. The `mgga_c_b94_kxc_unpol` wrapper below is the single
//! `#[cube]` entry point that calls each partN in order; partN modules are
//! NOT re-exported.
//!
//! Quick task 260514-q02 Plan 1: tests whether moving the partitioned files
//! under a single function module changes rustc / cubecl macro-fanout RSS
//! during `cargo check -p libxc-kernel-mgga-2`.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod part0;
mod part1;
mod part2;

use cubecl::prelude::*;

use part0::mgga_c_b94_kxc_unpol_part0;
use part1::mgga_c_b94_kxc_unpol_part1;
use part2::mgga_c_b94_kxc_unpol_part2;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_b94_kxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rho2lapl: &mut Array<f64>,
    v3rho2tau: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3rhosigmalapl: &mut Array<f64>,
    v3rhosigmatau: &mut Array<f64>,
    v3rholapl2: &mut Array<f64>,
    v3rholapltau: &mut Array<f64>,
    v3rhotau2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v3sigma2lapl: &mut Array<f64>,
    v3sigma2tau: &mut Array<f64>,
    v3sigmalapl2: &mut Array<f64>,
    v3sigmalapltau: &mut Array<f64>,
    v3sigmatau2: &mut Array<f64>,
    v3lapl3: &mut Array<f64>,
    v3lapl2tau: &mut Array<f64>,
    v3lapltau2: &mut Array<f64>,
    v3tau3: &mut Array<f64>,
    param_cab: f64,
    param_css: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_c_b94_kxc_unpol_part0(
        rho, sigma, lapl, tau,
        zk, vrho, vsigma, vlapl, vtau,
        v2rho2, v2rhosigma, v2rholapl, v2rhotau,
        v2sigma2, v2sigmalapl, v2sigmatau,
        v2lapl2, v2lapltau, v2tau2,
        v3rho3, v3rho2sigma, v3rho2lapl, v3rho2tau,
        v3rhosigma2, v3rhosigmalapl,
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
    mgga_c_b94_kxc_unpol_part1(
        rho, sigma, lapl, tau,
        v3rhosigmatau, v3rholapl2, v3rholapltau, v3rhotau2,
        v3sigma3, v3sigma2lapl, v3sigma2tau,
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
    mgga_c_b94_kxc_unpol_part2(
        rho, sigma, lapl, tau,
        v3sigmalapl2, v3sigmalapltau, v3sigmatau2,
        v3lapl3, v3lapl2tau, v3lapltau2, v3tau3,
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
}
