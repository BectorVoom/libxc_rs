//! GGA_X_GG99 lxc pol kernel — lxc_pol (nested-by-output, 5 parts).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod part0;
mod part1;
mod part2;
mod part3;
mod part4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_2};
use libxc_rkernel_math::special::{xc_dilogarithm};

use part0::gga_x_gg99_lxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2_v3rho3_v3rho2sigma_etc;
use part1::gga_x_gg99_lxc_pol_part1_v4rho4;
use part2::gga_x_gg99_lxc_pol_part2_v4rho3sigma;
use part3::gga_x_gg99_lxc_pol_part3_v4rho2sigma2;
use part4::gga_x_gg99_lxc_pol_part4_v4rhosigma3_v4sigma4;

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_gg99_lxc_pol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    gga_x_gg99_lxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2_v3rho3_v3rho2sigma_etc(rho, sigma, zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2, v3rho3, v3rho2sigma, v3rhosigma2, v3sigma3, dens_threshold, zeta_threshold);
    gga_x_gg99_lxc_pol_part1_v4rho4(rho, sigma, v4rho4, dens_threshold, zeta_threshold);
    gga_x_gg99_lxc_pol_part2_v4rho3sigma(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_x_gg99_lxc_pol_part3_v4rho2sigma2(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_x_gg99_lxc_pol_part4_v4rhosigma3_v4sigma4(rho, sigma, v4rhosigma3, v4sigma4, dens_threshold, zeta_threshold);
}
