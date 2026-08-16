//! MGGA_X_BR89_EXPLICIT kxc pol kernel — kxc_pol (nested-by-output, 13 parts).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

use part0::mgga_x_br89_explicit_kxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
use part1::mgga_x_br89_explicit_kxc_pol_part1_v3rho3_v3rho2sigma;
use part2::mgga_x_br89_explicit_kxc_pol_part2_v3rho2lapl;
use part3::mgga_x_br89_explicit_kxc_pol_part3_v3rho2tau_v3rhosigma2;
use part4::mgga_x_br89_explicit_kxc_pol_part4_v3rhosigmalapl;
use part5::mgga_x_br89_explicit_kxc_pol_part5_v3rhosigmatau;
use part6::mgga_x_br89_explicit_kxc_pol_part6_v3rholapl2;
use part7::mgga_x_br89_explicit_kxc_pol_part7_v3rholapltau;
use part8::mgga_x_br89_explicit_kxc_pol_part8_v3rhotau2_v3sigma3;
use part9::mgga_x_br89_explicit_kxc_pol_part9_v3sigma2lapl_v3sigma2tau;
use part10::mgga_x_br89_explicit_kxc_pol_part10_v3sigmalapl2_v3sigmalapltau;
use part11::mgga_x_br89_explicit_kxc_pol_part11_v3sigmatau2_v3lapl3;
use part12::mgga_x_br89_explicit_kxc_pol_part12_v3lapl2tau_v3lapltau2_v3tau3;

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_br89_explicit_kxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rho2lapl: &mut [f64],
    v3rho2tau: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3rhosigmalapl: &mut [f64],
    v3rhosigmatau: &mut [f64],
    v3rholapl2: &mut [f64],
    v3rholapltau: &mut [f64],
    v3rhotau2: &mut [f64],
    v3sigma3: &mut [f64],
    v3sigma2lapl: &mut [f64],
    v3sigma2tau: &mut [f64],
    v3sigmalapl2: &mut [f64],
    v3sigmalapltau: &mut [f64],
    v3sigmatau2: &mut [f64],
    v3lapl3: &mut [f64],
    v3lapl2tau: &mut [f64],
    v3lapltau2: &mut [f64],
    v3tau3: &mut [f64],
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_x_br89_explicit_kxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_kxc_pol_part1_v3rho3_v3rho2sigma(rho, sigma, lapl, tau, v3rho3, v3rho2sigma, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_kxc_pol_part2_v3rho2lapl(rho, sigma, lapl, tau, v3rho2lapl, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_kxc_pol_part3_v3rho2tau_v3rhosigma2(rho, sigma, lapl, tau, v3rho2tau, v3rhosigma2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_kxc_pol_part4_v3rhosigmalapl(rho, sigma, lapl, tau, v3rhosigmalapl, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_kxc_pol_part5_v3rhosigmatau(rho, sigma, lapl, tau, v3rhosigmatau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_kxc_pol_part6_v3rholapl2(rho, sigma, lapl, tau, v3rholapl2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_kxc_pol_part7_v3rholapltau(rho, sigma, lapl, tau, v3rholapltau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_kxc_pol_part8_v3rhotau2_v3sigma3(rho, sigma, lapl, tau, v3rhotau2, v3sigma3, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_kxc_pol_part9_v3sigma2lapl_v3sigma2tau(rho, sigma, lapl, tau, v3sigma2lapl, v3sigma2tau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_kxc_pol_part10_v3sigmalapl2_v3sigmalapltau(rho, sigma, lapl, tau, v3sigmalapl2, v3sigmalapltau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_kxc_pol_part11_v3sigmatau2_v3lapl3(rho, sigma, lapl, tau, v3sigmatau2, v3lapl3, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_kxc_pol_part12_v3lapl2tau_v3lapltau2_v3tau3(rho, sigma, lapl, tau, v3lapl2tau, v3lapltau2, v3tau3, param_gamma, dens_threshold, zeta_threshold);
}
