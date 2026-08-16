//! HYB_MGGA_XC_WB97MV kxc pol kernel — kxc_pol (nested-by-output, 5 parts).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod part0;
mod part1;
mod part2;
mod part3;
mod part4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use part0::hyb_mgga_xc_wb97mv_kxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
use part1::hyb_mgga_xc_wb97mv_kxc_pol_part1_v3rho3;
use part2::hyb_mgga_xc_wb97mv_kxc_pol_part2_v3rho2sigma_v3rho2lapl;
use part3::hyb_mgga_xc_wb97mv_kxc_pol_part3_v3rho2tau_v3rhosigma2_v3rhosigmalapl_v3rhosigmatau_v3rholapl_etc;
use part4::hyb_mgga_xc_wb97mv_kxc_pol_part4_v3rhotau2_v3sigma3_v3sigma2lapl_v3sigma2tau_v3sigmalapl2_v3s_etc;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_kxc_pol(
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
    param_c_os_0: f64,
    param_c_os_1: f64,
    param_c_os_2: f64,
    param_c_os_3: f64,
    param_c_os_4: f64,
    param_c_os_5: f64,
    param_c_ss_0: f64,
    param_c_ss_1: f64,
    param_c_ss_2: f64,
    param_c_ss_3: f64,
    param_c_ss_4: f64,
    param_c_x_0: f64,
    param_c_x_1: f64,
    param_c_x_2: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    hyb_mgga_xc_wb97mv_kxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, param_c_os_0, param_c_os_1, param_c_os_2, param_c_os_3, param_c_os_4, param_c_os_5, param_c_ss_0, param_c_ss_1, param_c_ss_2, param_c_ss_3, param_c_ss_4, param_c_x_0, param_c_x_1, param_c_x_2, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_xc_wb97mv_kxc_pol_part1_v3rho3(rho, sigma, lapl, tau, v3rho3, param_c_os_0, param_c_os_1, param_c_os_2, param_c_os_3, param_c_os_4, param_c_os_5, param_c_ss_0, param_c_ss_1, param_c_ss_2, param_c_ss_3, param_c_ss_4, param_c_x_0, param_c_x_1, param_c_x_2, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_xc_wb97mv_kxc_pol_part2_v3rho2sigma_v3rho2lapl(rho, sigma, lapl, tau, v3rho2sigma, v3rho2lapl, param_c_os_3, param_c_os_5, param_c_ss_1, param_c_ss_4, param_c_x_1, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_xc_wb97mv_kxc_pol_part3_v3rho2tau_v3rhosigma2_v3rhosigmalapl_v3rhosigmatau_v3rholapl_etc(rho, sigma, lapl, tau, v3rho2tau, v3rhosigma2, v3rhosigmalapl, v3rhosigmatau, v3rholapl2, v3rholapltau, param_c_os_1, param_c_os_2, param_c_os_3, param_c_os_4, param_c_os_5, param_c_ss_1, param_c_ss_2, param_c_ss_3, param_c_ss_4, param_c_x_1, param_c_x_2, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_xc_wb97mv_kxc_pol_part4_v3rhotau2_v3sigma3_v3sigma2lapl_v3sigma2tau_v3sigmalapl2_v3s_etc(rho, sigma, lapl, tau, v3rhotau2, v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2, v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3, param_c_os_1, param_c_os_2, param_c_os_3, param_c_os_4, param_c_os_5, param_c_ss_1, param_c_ss_2, param_c_ss_3, param_c_ss_4, param_c_x_1, param_c_x_2, param_hyb_omega_0, dens_threshold, zeta_threshold);
}
