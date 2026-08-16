//! GGA_C_FT97 kxc pol kernel — kxc_pol (nested-by-output, 11 parts).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

use part0::gga_c_ft97_kxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2;
use part1::gga_c_ft97_kxc_pol_part1_v3rho3_0;
use part2::gga_c_ft97_kxc_pol_part2_v3rho3_1;
use part3::gga_c_ft97_kxc_pol_part3_v3rho3_2;
use part4::gga_c_ft97_kxc_pol_part4_v3rho3_3;
use part5::gga_c_ft97_kxc_pol_part5_v3rho2sigma_0_v3rho2sigma_1_v3rho2sigma_2;
use part6::gga_c_ft97_kxc_pol_part6_v3rho2sigma_3_v3rho2sigma_4;
use part7::gga_c_ft97_kxc_pol_part7_v3rho2sigma_5_v3rho2sigma_6_v3rho2sigma_7;
use part8::gga_c_ft97_kxc_pol_part8_v3rho2sigma_8;
use part9::gga_c_ft97_kxc_pol_part9_v3rhosigma2;
use part10::gga_c_ft97_kxc_pol_part10_v3sigma3;

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_ft97_kxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    gga_c_ft97_kxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2(rho, sigma, zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2, dens_threshold, zeta_threshold);
    gga_c_ft97_kxc_pol_part1_v3rho3_0(rho, sigma, v3rho3, dens_threshold, zeta_threshold);
    gga_c_ft97_kxc_pol_part2_v3rho3_1(rho, sigma, v3rho3, dens_threshold, zeta_threshold);
    gga_c_ft97_kxc_pol_part3_v3rho3_2(rho, sigma, v3rho3, dens_threshold, zeta_threshold);
    gga_c_ft97_kxc_pol_part4_v3rho3_3(rho, sigma, v3rho3, dens_threshold, zeta_threshold);
    gga_c_ft97_kxc_pol_part5_v3rho2sigma_0_v3rho2sigma_1_v3rho2sigma_2(rho, sigma, v3rho2sigma, dens_threshold, zeta_threshold);
    gga_c_ft97_kxc_pol_part6_v3rho2sigma_3_v3rho2sigma_4(rho, sigma, v3rho2sigma, dens_threshold, zeta_threshold);
    gga_c_ft97_kxc_pol_part7_v3rho2sigma_5_v3rho2sigma_6_v3rho2sigma_7(rho, sigma, v3rho2sigma, dens_threshold, zeta_threshold);
    gga_c_ft97_kxc_pol_part8_v3rho2sigma_8(rho, sigma, v3rho2sigma, dens_threshold, zeta_threshold);
    gga_c_ft97_kxc_pol_part9_v3rhosigma2(rho, sigma, v3rhosigma2, dens_threshold, zeta_threshold);
    gga_c_ft97_kxc_pol_part10_v3sigma3(rho, sigma, v3sigma3, dens_threshold, zeta_threshold);
}
