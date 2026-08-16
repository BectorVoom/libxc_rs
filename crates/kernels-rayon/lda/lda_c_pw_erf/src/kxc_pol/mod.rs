//! LDA_C_PW_ERF kxc pol kernel — kxc_pol (nested-by-output, 2 parts).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod part0;
mod part1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

use part0::lda_c_pw_erf_kxc_pol_part0_zk_vrho_v2rho2;
use part1::lda_c_pw_erf_kxc_pol_part1_v3rho3;

/// LDA_C_PW_ERF kxc -- polarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_c_pw_erf_kxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    lda_c_pw_erf_kxc_pol_part0_zk_vrho_v2rho2(rho, zk, vrho, v2rho2, param_hyb_omega_0, dens_threshold, zeta_threshold);
    lda_c_pw_erf_kxc_pol_part1_v3rho3(rho, v3rho3, param_hyb_omega_0, dens_threshold, zeta_threshold);
}
