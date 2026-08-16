//! GGA_C_P86VWN lxc pol kernel — lxc_pol (nested-by-output, 3 parts).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod part0;
mod part1;
mod part2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

use part0::gga_c_p86vwn_lxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2_v3rho3_v3rho2sigma_etc;
use part1::gga_c_p86vwn_lxc_pol_part1_v4rho4_v4rho3sigma_v4rho2sigma2;
use part2::gga_c_p86vwn_lxc_pol_part2_v4rhosigma3_v4sigma4;

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_p86vwn_lxc_pol(
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
    param_aa: f64,
    param_bb: f64,
    param_ftilde: f64,
    param_malpha: f64,
    param_mbeta: f64,
    param_mdelta: f64,
    param_mgamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    gga_c_p86vwn_lxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2_v3rho3_v3rho2sigma_etc(rho, sigma, zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2, v3rho3, v3rho2sigma, v3rhosigma2, v3sigma3, param_aa, param_bb, param_ftilde, param_malpha, param_mbeta, param_mdelta, param_mgamma, dens_threshold, zeta_threshold);
    gga_c_p86vwn_lxc_pol_part1_v4rho4_v4rho3sigma_v4rho2sigma2(rho, sigma, v4rho4, v4rho3sigma, v4rho2sigma2, param_aa, param_bb, param_ftilde, param_malpha, param_mbeta, param_mdelta, param_mgamma, dens_threshold, zeta_threshold);
    gga_c_p86vwn_lxc_pol_part2_v4rhosigma3_v4sigma4(rho, sigma, v4rhosigma3, v4sigma4, param_aa, param_bb, param_ftilde, param_malpha, param_mbeta, param_mdelta, param_mgamma, dens_threshold, zeta_threshold);
}
