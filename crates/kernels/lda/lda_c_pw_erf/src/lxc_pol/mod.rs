//! LDA_C_PW_ERF lxc pol kernel — lxc_pol (nested-by-output, 7 parts).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod part0;
mod part1;
mod part2;
mod part3;
mod part4;
mod part5;
mod part6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

use part0::lda_c_pw_erf_lxc_pol_part0_zk_vrho_v2rho2;
use part1::lda_c_pw_erf_lxc_pol_part1_v3rho3;
use part2::lda_c_pw_erf_lxc_pol_part2_v4rho4_0;
use part3::lda_c_pw_erf_lxc_pol_part3_v4rho4_1;
use part4::lda_c_pw_erf_lxc_pol_part4_v4rho4_2;
use part5::lda_c_pw_erf_lxc_pol_part5_v4rho4_3;
use part6::lda_c_pw_erf_lxc_pol_part6_v4rho4_4;

/// LDA_C_PW_ERF lxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    lda_c_pw_erf_lxc_pol_part0_zk_vrho_v2rho2(rho, zk, vrho, v2rho2, param_hyb_omega_0, dens_threshold, zeta_threshold);
    lda_c_pw_erf_lxc_pol_part1_v3rho3(rho, v3rho3, param_hyb_omega_0, dens_threshold, zeta_threshold);
    lda_c_pw_erf_lxc_pol_part2_v4rho4_0(rho, v4rho4, param_hyb_omega_0, dens_threshold, zeta_threshold);
    lda_c_pw_erf_lxc_pol_part3_v4rho4_1(rho, v4rho4, dens_threshold, zeta_threshold);
    lda_c_pw_erf_lxc_pol_part4_v4rho4_2(rho, v4rho4, param_hyb_omega_0, dens_threshold, zeta_threshold);
    lda_c_pw_erf_lxc_pol_part5_v4rho4_3(rho, v4rho4, dens_threshold, zeta_threshold);
    lda_c_pw_erf_lxc_pol_part6_v4rho4_4(rho, v4rho4, param_hyb_omega_0, dens_threshold, zeta_threshold);
}
