//! MGGA_X_BR89_EXPLICIT lxc unpol kernel — lxc_unpol (nested-by-output, 19 parts).
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
mod part16;
mod part17;
mod part18;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

use part0::mgga_x_br89_explicit_lxc_unpol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
use part1::mgga_x_br89_explicit_lxc_unpol_part1_v3rhosigmatau_v3rholapl2_v3rholapltau_v3rhotau2_v3sigma3_v3s_etc;
use part2::mgga_x_br89_explicit_lxc_unpol_part2_v3sigmalapl2_v3sigmalapltau_v3sigmatau2_v3lapl3_v3lapl2tau_v_etc;
use part3::mgga_x_br89_explicit_lxc_unpol_part3_v4rho3sigma_v4rho3lapl_v4rho3tau;
use part4::mgga_x_br89_explicit_lxc_unpol_part4_v4rho2sigma2_v4rho2sigmalapl;
use part5::mgga_x_br89_explicit_lxc_unpol_part5_v4rho2sigmatau_v4rho2lapl2;
use part6::mgga_x_br89_explicit_lxc_unpol_part6_v4rho2lapltau_v4rho2tau2;
use part7::mgga_x_br89_explicit_lxc_unpol_part7_v4rhosigma3_v4rhosigma2lapl;
use part8::mgga_x_br89_explicit_lxc_unpol_part8_v4rhosigma2tau;
use part9::mgga_x_br89_explicit_lxc_unpol_part9_v4rhosigmalapl2;
use part10::mgga_x_br89_explicit_lxc_unpol_part10_v4rhosigmalapltau;
use part11::mgga_x_br89_explicit_lxc_unpol_part11_v4rhosigmatau2_v4rholapl3;
use part12::mgga_x_br89_explicit_lxc_unpol_part12_v4rholapl2tau;
use part13::mgga_x_br89_explicit_lxc_unpol_part13_v4rholapltau2_v4rhotau3_v4sigma4;
use part14::mgga_x_br89_explicit_lxc_unpol_part14_v4sigma3lapl_v4sigma3tau_v4sigma2lapl2;
use part15::mgga_x_br89_explicit_lxc_unpol_part15_v4sigma2lapltau_v4sigma2tau2;
use part16::mgga_x_br89_explicit_lxc_unpol_part16_v4sigmalapl3_v4sigmalapl2tau;
use part17::mgga_x_br89_explicit_lxc_unpol_part17_v4sigmalapltau2_v4sigmatau3_v4lapl4;
use part18::mgga_x_br89_explicit_lxc_unpol_part18_v4lapl3tau_v4lapl2tau2_v4lapltau3_v4tau4;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_br89_explicit_lxc_unpol(
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
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho3lapl: &mut Array<f64>,
    v4rho3tau: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rho2sigmalapl: &mut Array<f64>,
    v4rho2sigmatau: &mut Array<f64>,
    v4rho2lapl2: &mut Array<f64>,
    v4rho2lapltau: &mut Array<f64>,
    v4rho2tau2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4rhosigma2lapl: &mut Array<f64>,
    v4rhosigma2tau: &mut Array<f64>,
    v4rhosigmalapl2: &mut Array<f64>,
    v4rhosigmalapltau: &mut Array<f64>,
    v4rhosigmatau2: &mut Array<f64>,
    v4rholapl3: &mut Array<f64>,
    v4rholapl2tau: &mut Array<f64>,
    v4rholapltau2: &mut Array<f64>,
    v4rhotau3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    v4sigma3lapl: &mut Array<f64>,
    v4sigma3tau: &mut Array<f64>,
    v4sigma2lapl2: &mut Array<f64>,
    v4sigma2lapltau: &mut Array<f64>,
    v4sigma2tau2: &mut Array<f64>,
    v4sigmalapl3: &mut Array<f64>,
    v4sigmalapl2tau: &mut Array<f64>,
    v4sigmalapltau2: &mut Array<f64>,
    v4sigmatau3: &mut Array<f64>,
    v4lapl4: &mut Array<f64>,
    v4lapl3tau: &mut Array<f64>,
    v4lapl2tau2: &mut Array<f64>,
    v4lapltau3: &mut Array<f64>,
    v4tau4: &mut Array<f64>,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_x_br89_explicit_lxc_unpol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, v3rho3, v3rho2sigma, v3rho2lapl, v3rho2tau, v3rhosigma2, v3rhosigmalapl, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_unpol_part1_v3rhosigmatau_v3rholapl2_v3rholapltau_v3rhotau2_v3sigma3_v3s_etc(rho, sigma, lapl, tau, v3rhosigmatau, v3rholapl2, v3rholapltau, v3rhotau2, v3sigma3, v3sigma2lapl, v3sigma2tau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_unpol_part2_v3sigmalapl2_v3sigmalapltau_v3sigmatau2_v3lapl3_v3lapl2tau_v_etc(rho, sigma, lapl, tau, v3sigmalapl2, v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3, v4rho4, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_unpol_part3_v4rho3sigma_v4rho3lapl_v4rho3tau(rho, sigma, lapl, tau, v4rho3sigma, v4rho3lapl, v4rho3tau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_unpol_part4_v4rho2sigma2_v4rho2sigmalapl(rho, sigma, lapl, tau, v4rho2sigma2, v4rho2sigmalapl, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_unpol_part5_v4rho2sigmatau_v4rho2lapl2(rho, sigma, lapl, tau, v4rho2sigmatau, v4rho2lapl2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_unpol_part6_v4rho2lapltau_v4rho2tau2(rho, sigma, lapl, tau, v4rho2lapltau, v4rho2tau2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_unpol_part7_v4rhosigma3_v4rhosigma2lapl(rho, sigma, lapl, tau, v4rhosigma3, v4rhosigma2lapl, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_unpol_part8_v4rhosigma2tau(rho, sigma, lapl, tau, v4rhosigma2tau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_unpol_part9_v4rhosigmalapl2(rho, sigma, lapl, tau, v4rhosigmalapl2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_unpol_part10_v4rhosigmalapltau(rho, sigma, lapl, tau, v4rhosigmalapltau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_unpol_part11_v4rhosigmatau2_v4rholapl3(rho, sigma, lapl, tau, v4rhosigmatau2, v4rholapl3, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_unpol_part12_v4rholapl2tau(rho, sigma, lapl, tau, v4rholapl2tau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_unpol_part13_v4rholapltau2_v4rhotau3_v4sigma4(rho, sigma, lapl, tau, v4rholapltau2, v4rhotau3, v4sigma4, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_unpol_part14_v4sigma3lapl_v4sigma3tau_v4sigma2lapl2(rho, sigma, lapl, tau, v4sigma3lapl, v4sigma3tau, v4sigma2lapl2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_unpol_part15_v4sigma2lapltau_v4sigma2tau2(rho, sigma, lapl, tau, v4sigma2lapltau, v4sigma2tau2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_unpol_part16_v4sigmalapl3_v4sigmalapl2tau(rho, sigma, lapl, tau, v4sigmalapl3, v4sigmalapl2tau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_unpol_part17_v4sigmalapltau2_v4sigmatau3_v4lapl4(rho, sigma, lapl, tau, v4sigmalapltau2, v4sigmatau3, v4lapl4, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_unpol_part18_v4lapl3tau_v4lapl2tau2_v4lapltau3_v4tau4(rho, sigma, lapl, tau, v4lapl3tau, v4lapl2tau2, v4lapltau3, v4tau4, param_gamma, dens_threshold, zeta_threshold);
}
