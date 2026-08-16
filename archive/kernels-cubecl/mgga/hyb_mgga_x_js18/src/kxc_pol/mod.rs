//! HYB_MGGA_X_JS18 kxc pol kernel — kxc_pol (nested-by-output, 9 parts).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

use part0::hyb_mgga_x_js18_kxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2;
use part1::hyb_mgga_x_js18_kxc_pol_part1_v2rhosigma_v2rholapl_v2rhotau_v2sigma2_v2sigmalapl_v2sigmata_etc;
use part2::hyb_mgga_x_js18_kxc_pol_part2_v3rho3;
use part3::hyb_mgga_x_js18_kxc_pol_part3_v3rho2sigma_0_v3rho2sigma_1_v3rho2sigma_2_v3rho2sigma_3_v3rh_etc;
use part4::hyb_mgga_x_js18_kxc_pol_part4_v3rho2sigma_5_v3rho2sigma_6_v3rho2sigma_7_v3rho2sigma_8;
use part5::hyb_mgga_x_js18_kxc_pol_part5_v3rho2lapl_v3rho2tau;
use part6::hyb_mgga_x_js18_kxc_pol_part6_v3rhosigma2_v3rhosigmalapl;
use part7::hyb_mgga_x_js18_kxc_pol_part7_v3rhosigmatau_v3rholapl2_v3rholapltau_v3rhotau2;
use part8::hyb_mgga_x_js18_kxc_pol_part8_v3sigma3_v3sigma2lapl_v3sigma2tau_v3sigmalapl2_v3sigmalaplta_etc;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn hyb_mgga_x_js18_kxc_pol(
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
    param_hyb_coeff_0: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    hyb_mgga_x_js18_kxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_kxc_pol_part1_v2rhosigma_v2rholapl_v2rhotau_v2sigma2_v2sigmalapl_v2sigmata_etc(rho, sigma, lapl, tau, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_kxc_pol_part2_v3rho3(rho, sigma, lapl, tau, v3rho3, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_kxc_pol_part3_v3rho2sigma_0_v3rho2sigma_1_v3rho2sigma_2_v3rho2sigma_3_v3rh_etc(rho, sigma, lapl, tau, v3rho2sigma, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_kxc_pol_part4_v3rho2sigma_5_v3rho2sigma_6_v3rho2sigma_7_v3rho2sigma_8(rho, sigma, lapl, tau, v3rho2sigma, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_kxc_pol_part5_v3rho2lapl_v3rho2tau(rho, sigma, lapl, tau, v3rho2lapl, v3rho2tau, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_kxc_pol_part6_v3rhosigma2_v3rhosigmalapl(rho, sigma, lapl, tau, v3rhosigma2, v3rhosigmalapl, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_kxc_pol_part7_v3rhosigmatau_v3rholapl2_v3rholapltau_v3rhotau2(rho, sigma, lapl, tau, v3rhosigmatau, v3rholapl2, v3rholapltau, v3rhotau2, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_kxc_pol_part8_v3sigma3_v3sigma2lapl_v3sigma2tau_v3sigmalapl2_v3sigmalaplta_etc(rho, sigma, lapl, tau, v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2, v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
}
