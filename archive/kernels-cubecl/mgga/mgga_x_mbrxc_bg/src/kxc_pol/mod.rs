//! MGGA_X_MBRXC_BG kxc pol kernel — kxc_pol (nested-by-output, 6 parts).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod part0;
mod part1;
mod part2;
mod part3;
mod part4;
mod part5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::mbrxc::{xc_mgga_x_mbrxc_get_x};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};

use part0::mgga_x_mbrxc_bg_kxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
use part1::mgga_x_mbrxc_bg_kxc_pol_part1_v3rho2sigma_v3rho2lapl_v3rho2tau;
use part2::mgga_x_mbrxc_bg_kxc_pol_part2_v3rhosigma2_v3rhosigmalapl;
use part3::mgga_x_mbrxc_bg_kxc_pol_part3_v3rhosigmatau_v3rholapl2_v3rholapltau;
use part4::mgga_x_mbrxc_bg_kxc_pol_part4_v3rhotau2_v3sigma3_v3sigma2lapl;
use part5::mgga_x_mbrxc_bg_kxc_pol_part5_v3sigma2tau_v3sigmalapl2_v3sigmalapltau_v3sigmatau2_v3lapl3__etc;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_mbrxc_bg_kxc_pol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_x_mbrxc_bg_kxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, v3rho3, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_kxc_pol_part1_v3rho2sigma_v3rho2lapl_v3rho2tau(rho, sigma, lapl, tau, v3rho2sigma, v3rho2lapl, v3rho2tau, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_kxc_pol_part2_v3rhosigma2_v3rhosigmalapl(rho, sigma, lapl, tau, v3rhosigma2, v3rhosigmalapl, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_kxc_pol_part3_v3rhosigmatau_v3rholapl2_v3rholapltau(rho, sigma, lapl, tau, v3rhosigmatau, v3rholapl2, v3rholapltau, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_kxc_pol_part4_v3rhotau2_v3sigma3_v3sigma2lapl(rho, sigma, lapl, tau, v3rhotau2, v3sigma3, v3sigma2lapl, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_kxc_pol_part5_v3sigma2tau_v3sigmalapl2_v3sigmalapltau_v3sigmatau2_v3lapl3__etc(rho, sigma, lapl, tau, v3sigma2tau, v3sigmalapl2, v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3, dens_threshold, zeta_threshold);
}
