//! GGA_X_WPBEH lxc pol kernel — lxc_pol (nested-by-output, 33 parts).
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
mod part19;
mod part20;
mod part21;
mod part22;
mod part23;
mod part24;
mod part25;
mod part26;
mod part27;
mod part28;
mod part29;
mod part30;
mod part31;
mod part32;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::special::{xc_erfcx};

use part0::gga_x_wpbeh_lxc_pol_part0_zk_vrho_vsigma_v2rho2;
use part1::gga_x_wpbeh_lxc_pol_part1_v2rhosigma_v2sigma2;
use part2::gga_x_wpbeh_lxc_pol_part2_v3rho3_0;
use part3::gga_x_wpbeh_lxc_pol_part3_v3rho3_1;
use part4::gga_x_wpbeh_lxc_pol_part4_v3rho3_2;
use part5::gga_x_wpbeh_lxc_pol_part5_v3rho3_3;
use part6::gga_x_wpbeh_lxc_pol_part6_v3rho2sigma_0_v3rho2sigma_1_v3rho2sigma_2;
use part7::gga_x_wpbeh_lxc_pol_part7_v3rho2sigma_3_v3rho2sigma_4_v3rho2sigma_5;
use part8::gga_x_wpbeh_lxc_pol_part8_v3rho2sigma_6_v3rho2sigma_7_v3rho2sigma_8;
use part9::gga_x_wpbeh_lxc_pol_part9_v3rhosigma2;
use part10::gga_x_wpbeh_lxc_pol_part10_v3sigma3;
use part11::gga_x_wpbeh_lxc_pol_part11_v4rho4_0;
use part12::gga_x_wpbeh_lxc_pol_part12_v4rho4_1;
use part13::gga_x_wpbeh_lxc_pol_part13_v4rho4_2;
use part14::gga_x_wpbeh_lxc_pol_part14_v4rho4_3;
use part15::gga_x_wpbeh_lxc_pol_part15_v4rho4_4;
use part16::gga_x_wpbeh_lxc_pol_part16_v4rho3sigma_0_v4rho3sigma_1;
use part17::gga_x_wpbeh_lxc_pol_part17_v4rho3sigma_2;
use part18::gga_x_wpbeh_lxc_pol_part18_v4rho3sigma_3_v4rho3sigma_4;
use part19::gga_x_wpbeh_lxc_pol_part19_v4rho3sigma_5;
use part20::gga_x_wpbeh_lxc_pol_part20_v4rho3sigma_6_v4rho3sigma_7;
use part21::gga_x_wpbeh_lxc_pol_part21_v4rho3sigma_8;
use part22::gga_x_wpbeh_lxc_pol_part22_v4rho3sigma_9_v4rho3sigma_10;
use part23::gga_x_wpbeh_lxc_pol_part23_v4rho3sigma_11;
use part24::gga_x_wpbeh_lxc_pol_part24_v4rho2sigma2_0_v4rho2sigma2_1_v4rho2sigma2_2_v4rho2sigma2_3__etc;
use part25::gga_x_wpbeh_lxc_pol_part25_v4rho2sigma2_5;
use part26::gga_x_wpbeh_lxc_pol_part26_v4rho2sigma2_6_v4rho2sigma2_7_v4rho2sigma2_8_v4rho2sigma2_9__etc;
use part27::gga_x_wpbeh_lxc_pol_part27_v4rho2sigma2_11;
use part28::gga_x_wpbeh_lxc_pol_part28_v4rho2sigma2_12_v4rho2sigma2_13_v4rho2sigma2_14_v4rho2sigma2_etc;
use part29::gga_x_wpbeh_lxc_pol_part29_v4rho2sigma2_17;
use part30::gga_x_wpbeh_lxc_pol_part30_v4rhosigma3_0_v4rhosigma3_1_v4rhosigma3_2_v4rhosigma3_3_v4rh_etc;
use part31::gga_x_wpbeh_lxc_pol_part31_v4rhosigma3_10_v4rhosigma3_11_v4rhosigma3_12_v4rhosigma3_13__etc;
use part32::gga_x_wpbeh_lxc_pol_part32_v4sigma4;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_wpbeh_lxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    gga_x_wpbeh_lxc_pol_part0_zk_vrho_vsigma_v2rho2(rho, sigma, zk, vrho, vsigma, v2rho2, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part1_v2rhosigma_v2sigma2(rho, sigma, v2rhosigma, v2sigma2, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part2_v3rho3_0(rho, sigma, v3rho3, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part3_v3rho3_1(rho, sigma, v3rho3, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part4_v3rho3_2(rho, sigma, v3rho3, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part5_v3rho3_3(rho, sigma, v3rho3, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part6_v3rho2sigma_0_v3rho2sigma_1_v3rho2sigma_2(rho, sigma, v3rho2sigma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part7_v3rho2sigma_3_v3rho2sigma_4_v3rho2sigma_5(rho, sigma, v3rho2sigma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part8_v3rho2sigma_6_v3rho2sigma_7_v3rho2sigma_8(rho, sigma, v3rho2sigma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part9_v3rhosigma2(rho, sigma, v3rhosigma2, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part10_v3sigma3(rho, sigma, v3sigma3, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part11_v4rho4_0(rho, sigma, v4rho4, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part12_v4rho4_1(rho, sigma, v4rho4, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part13_v4rho4_2(rho, sigma, v4rho4, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part14_v4rho4_3(rho, sigma, v4rho4, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part15_v4rho4_4(rho, sigma, v4rho4, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part16_v4rho3sigma_0_v4rho3sigma_1(rho, sigma, v4rho3sigma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part17_v4rho3sigma_2(rho, sigma, v4rho3sigma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part18_v4rho3sigma_3_v4rho3sigma_4(rho, sigma, v4rho3sigma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part19_v4rho3sigma_5(rho, sigma, v4rho3sigma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part20_v4rho3sigma_6_v4rho3sigma_7(rho, sigma, v4rho3sigma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part21_v4rho3sigma_8(rho, sigma, v4rho3sigma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part22_v4rho3sigma_9_v4rho3sigma_10(rho, sigma, v4rho3sigma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part23_v4rho3sigma_11(rho, sigma, v4rho3sigma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part24_v4rho2sigma2_0_v4rho2sigma2_1_v4rho2sigma2_2_v4rho2sigma2_3__etc(rho, sigma, v4rho2sigma2, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part25_v4rho2sigma2_5(rho, sigma, v4rho2sigma2, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part26_v4rho2sigma2_6_v4rho2sigma2_7_v4rho2sigma2_8_v4rho2sigma2_9__etc(rho, sigma, v4rho2sigma2, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part27_v4rho2sigma2_11(rho, sigma, v4rho2sigma2, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part28_v4rho2sigma2_12_v4rho2sigma2_13_v4rho2sigma2_14_v4rho2sigma2_etc(rho, sigma, v4rho2sigma2, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part29_v4rho2sigma2_17(rho, sigma, v4rho2sigma2, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part30_v4rhosigma3_0_v4rhosigma3_1_v4rhosigma3_2_v4rhosigma3_3_v4rh_etc(rho, sigma, v4rhosigma3, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part31_v4rhosigma3_10_v4rhosigma3_11_v4rhosigma3_12_v4rhosigma3_13__etc(rho, sigma, v4rhosigma3, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_lxc_pol_part32_v4sigma4(rho, sigma, v4sigma4, param_hyb_omega_0, dens_threshold, zeta_threshold);
}
