//! MGGA_X_BR89 lxc unpol kernel — lxc_unpol (nested-by-output, 13 parts).
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

use libxc_rkernel_math::br89::{xc_mgga_x_br89_get_x};
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

use part0::mgga_x_br89_lxc_unpol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
use part1::mgga_x_br89_lxc_unpol_part1_v4rho3sigma_v4rho3lapl_v4rho3tau_v4rho2sigma2;
use part2::mgga_x_br89_lxc_unpol_part2_v4rho2sigmalapl_v4rho2sigmatau_v4rho2lapl2;
use part3::mgga_x_br89_lxc_unpol_part3_v4rho2lapltau_v4rho2tau2_v4rhosigma3;
use part4::mgga_x_br89_lxc_unpol_part4_v4rhosigma2lapl_v4rhosigma2tau;
use part5::mgga_x_br89_lxc_unpol_part5_v4rhosigmalapl2;
use part6::mgga_x_br89_lxc_unpol_part6_v4rhosigmalapltau;
use part7::mgga_x_br89_lxc_unpol_part7_v4rhosigmatau2_v4rholapl3;
use part8::mgga_x_br89_lxc_unpol_part8_v4rholapl2tau_v4rholapltau2;
use part9::mgga_x_br89_lxc_unpol_part9_v4rhotau3_v4sigma4_v4sigma3lapl_v4sigma3tau_v4sigma2lapl2;
use part10::mgga_x_br89_lxc_unpol_part10_v4sigma2lapltau_v4sigma2tau2_v4sigmalapl3;
use part11::mgga_x_br89_lxc_unpol_part11_v4sigmalapl2tau_v4sigmalapltau2_v4sigmatau3_v4lapl4;
use part12::mgga_x_br89_lxc_unpol_part12_v4lapl3tau_v4lapl2tau2_v4lapltau3_v4tau4;

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_br89_lxc_unpol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho3lapl: &mut [f64],
    v4rho3tau: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rho2sigmalapl: &mut [f64],
    v4rho2sigmatau: &mut [f64],
    v4rho2lapl2: &mut [f64],
    v4rho2lapltau: &mut [f64],
    v4rho2tau2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4rhosigma2lapl: &mut [f64],
    v4rhosigma2tau: &mut [f64],
    v4rhosigmalapl2: &mut [f64],
    v4rhosigmalapltau: &mut [f64],
    v4rhosigmatau2: &mut [f64],
    v4rholapl3: &mut [f64],
    v4rholapl2tau: &mut [f64],
    v4rholapltau2: &mut [f64],
    v4rhotau3: &mut [f64],
    v4sigma4: &mut [f64],
    v4sigma3lapl: &mut [f64],
    v4sigma3tau: &mut [f64],
    v4sigma2lapl2: &mut [f64],
    v4sigma2lapltau: &mut [f64],
    v4sigma2tau2: &mut [f64],
    v4sigmalapl3: &mut [f64],
    v4sigmalapl2tau: &mut [f64],
    v4sigmalapltau2: &mut [f64],
    v4sigmatau3: &mut [f64],
    v4lapl4: &mut [f64],
    v4lapl3tau: &mut [f64],
    v4lapl2tau2: &mut [f64],
    v4lapltau3: &mut [f64],
    v4tau4: &mut [f64],
    param_at: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_x_br89_lxc_unpol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, v3rho3, v3rho2sigma, v3rho2lapl, v3rho2tau, v3rhosigma2, v3rhosigmalapl, v3rhosigmatau, v3rholapl2, v3rholapltau, v3rhotau2, v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2, v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3, v4rho4, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_unpol_part1_v4rho3sigma_v4rho3lapl_v4rho3tau_v4rho2sigma2(rho, sigma, lapl, tau, v4rho3sigma, v4rho3lapl, v4rho3tau, v4rho2sigma2, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_unpol_part2_v4rho2sigmalapl_v4rho2sigmatau_v4rho2lapl2(rho, sigma, lapl, tau, v4rho2sigmalapl, v4rho2sigmatau, v4rho2lapl2, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_unpol_part3_v4rho2lapltau_v4rho2tau2_v4rhosigma3(rho, sigma, lapl, tau, v4rho2lapltau, v4rho2tau2, v4rhosigma3, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_unpol_part4_v4rhosigma2lapl_v4rhosigma2tau(rho, sigma, lapl, tau, v4rhosigma2lapl, v4rhosigma2tau, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_unpol_part5_v4rhosigmalapl2(rho, sigma, lapl, tau, v4rhosigmalapl2, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_unpol_part6_v4rhosigmalapltau(rho, sigma, lapl, tau, v4rhosigmalapltau, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_unpol_part7_v4rhosigmatau2_v4rholapl3(rho, sigma, lapl, tau, v4rhosigmatau2, v4rholapl3, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_unpol_part8_v4rholapl2tau_v4rholapltau2(rho, sigma, lapl, tau, v4rholapl2tau, v4rholapltau2, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_unpol_part9_v4rhotau3_v4sigma4_v4sigma3lapl_v4sigma3tau_v4sigma2lapl2(rho, sigma, lapl, tau, v4rhotau3, v4sigma4, v4sigma3lapl, v4sigma3tau, v4sigma2lapl2, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_unpol_part10_v4sigma2lapltau_v4sigma2tau2_v4sigmalapl3(rho, sigma, lapl, tau, v4sigma2lapltau, v4sigma2tau2, v4sigmalapl3, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_unpol_part11_v4sigmalapl2tau_v4sigmalapltau2_v4sigmatau3_v4lapl4(rho, sigma, lapl, tau, v4sigmalapl2tau, v4sigmalapltau2, v4sigmatau3, v4lapl4, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_unpol_part12_v4lapl3tau_v4lapl2tau2_v4lapltau3_v4tau4(rho, sigma, lapl, tau, v4lapl3tau, v4lapl2tau2, v4lapltau3, v4tau4, param_at, param_gamma, dens_threshold, zeta_threshold);
}
