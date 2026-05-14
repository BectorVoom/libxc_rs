//! MGGA_C_B94 polarized kxc-level kernel.
//!
//! The Maple2c-translated body is split across 16 private internal modules
//! `part0`..`part15`. The `mgga_c_b94_kxc_pol` wrapper below is the single
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
mod part3;
mod part4;
mod part5;
mod part6;
mod part7;
mod part8;
mod part9;
mod part10;
mod part11;
mod part12;
mod part13;
mod part14;
mod part15;

use cubecl::prelude::*;

use part0::mgga_c_b94_kxc_pol_part0;
use part1::mgga_c_b94_kxc_pol_part1;
use part2::mgga_c_b94_kxc_pol_part2;
use part3::mgga_c_b94_kxc_pol_part3;
use part4::mgga_c_b94_kxc_pol_part4;
use part5::mgga_c_b94_kxc_pol_part5;
use part6::mgga_c_b94_kxc_pol_part6;
use part7::mgga_c_b94_kxc_pol_part7;
use part8::mgga_c_b94_kxc_pol_part8;
use part9::mgga_c_b94_kxc_pol_part9;
use part10::mgga_c_b94_kxc_pol_part10;
use part11::mgga_c_b94_kxc_pol_part11;
use part12::mgga_c_b94_kxc_pol_part12;
use part13::mgga_c_b94_kxc_pol_part13;
use part14::mgga_c_b94_kxc_pol_part14;
use part15::mgga_c_b94_kxc_pol_part15;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_b94_kxc_pol(
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
    mgga_c_b94_kxc_pol_part0(
        rho, sigma, lapl, tau,
        zk, vrho, vsigma, vlapl, vtau,
        v2rho2, v2rhosigma, v2rholapl, v2rhotau,
        v2sigma2, v2sigmalapl, v2sigmatau,
        v2lapl2, v2lapltau, v2tau2,
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
    mgga_c_b94_kxc_pol_part1(
        rho, sigma, lapl, tau, v3rho3,
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
    mgga_c_b94_kxc_pol_part2(
        rho, sigma, lapl, tau, v3rho2sigma,
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
    mgga_c_b94_kxc_pol_part3(
        rho, sigma, lapl, tau, v3rho2lapl,
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
    mgga_c_b94_kxc_pol_part4(
        rho, sigma, lapl, tau, v3rho2tau,
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
    mgga_c_b94_kxc_pol_part5(
        rho, sigma, lapl, tau, v3rhosigma2,
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
    mgga_c_b94_kxc_pol_part6(
        rho, sigma, lapl, tau, v3rhosigmalapl,
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
    mgga_c_b94_kxc_pol_part7(
        rho, sigma, lapl, tau, v3rhosigmatau,
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
    mgga_c_b94_kxc_pol_part8(
        rho, sigma, lapl, tau, v3rholapl2,
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
    mgga_c_b94_kxc_pol_part9(
        rho, sigma, lapl, tau, v3rholapltau,
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
    mgga_c_b94_kxc_pol_part10(
        rho, sigma, lapl, tau, v3rhotau2, v3sigma3,
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
    mgga_c_b94_kxc_pol_part11(
        rho, sigma, lapl, tau, v3sigma2lapl, v3sigma2tau,
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
    mgga_c_b94_kxc_pol_part12(
        rho, sigma, lapl, tau, v3sigmalapl2,
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
    mgga_c_b94_kxc_pol_part13(
        rho, sigma, lapl, tau, v3sigmalapltau,
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
    mgga_c_b94_kxc_pol_part14(
        rho, sigma, lapl, tau, v3sigmatau2, v3lapl3,
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
    mgga_c_b94_kxc_pol_part15(
        rho, sigma, lapl, tau, v3lapl2tau, v3lapltau2, v3tau3,
        param_cab, param_css, param_gamma, dens_threshold, zeta_threshold,
    );
}
