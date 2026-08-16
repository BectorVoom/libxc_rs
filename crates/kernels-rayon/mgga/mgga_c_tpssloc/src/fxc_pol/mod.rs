//! MGGA_C_TPSSLOC fxc pol kernel — fxc_pol (nested-by-output, 2 parts).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod part0;
mod part1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use part0::mgga_c_tpssloc_fxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2;
use part1::mgga_c_tpssloc_fxc_pol_part1_v2rhosigma_v2rholapl_v2rhotau_v2sigma2_v2sigmalapl_v2sigmata_etc;

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_tpssloc_fxc_pol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_c_tpssloc_fxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, dens_threshold, zeta_threshold);
    mgga_c_tpssloc_fxc_pol_part1_v2rhosigma_v2rholapl_v2rhotau_v2sigma2_v2sigmalapl_v2sigmata_etc(rho, sigma, lapl, tau, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, dens_threshold, zeta_threshold);
}
