//! LDA_XC_KSDT lxc pol kernel — lxc_pol (nested-by-output, 10 parts).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3};
use libxc_rkernel_math::piecewise::{piecewise3};

use part0::lda_xc_ksdt_lxc_pol_part0_zk_vrho_v2rho2;
use part1::lda_xc_ksdt_lxc_pol_part1_v3rho3_0;
use part2::lda_xc_ksdt_lxc_pol_part2_v3rho3_1;
use part3::lda_xc_ksdt_lxc_pol_part3_v3rho3_2;
use part4::lda_xc_ksdt_lxc_pol_part4_v3rho3_3;
use part5::lda_xc_ksdt_lxc_pol_part5_v4rho4_0;
use part6::lda_xc_ksdt_lxc_pol_part6_v4rho4_1;
use part7::lda_xc_ksdt_lxc_pol_part7_v4rho4_2;
use part8::lda_xc_ksdt_lxc_pol_part8_v4rho4_3;
use part9::lda_xc_ksdt_lxc_pol_part9_v4rho4_4;

/// LDA_XC_KSDT lxc -- polarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_xc_ksdt_lxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
    param_T: f64,
    param_b_0_0: f64,
    param_b_0_1: f64,
    param_b_0_2: f64,
    param_b_0_3: f64,
    param_b_0_4: f64,
    param_b_1_0: f64,
    param_b_1_1: f64,
    param_b_1_2: f64,
    param_b_1_3: f64,
    param_b_1_4: f64,
    param_c_0_0: f64,
    param_c_0_1: f64,
    param_c_0_2: f64,
    param_c_1_0: f64,
    param_c_1_1: f64,
    param_c_1_2: f64,
    param_d_0_0: f64,
    param_d_0_1: f64,
    param_d_0_2: f64,
    param_d_0_3: f64,
    param_d_0_4: f64,
    param_d_1_0: f64,
    param_d_1_1: f64,
    param_d_1_2: f64,
    param_d_1_3: f64,
    param_d_1_4: f64,
    param_e_0_0: f64,
    param_e_0_1: f64,
    param_e_0_2: f64,
    param_e_0_3: f64,
    param_e_0_4: f64,
    param_e_1_0: f64,
    param_e_1_1: f64,
    param_e_1_2: f64,
    param_e_1_3: f64,
    param_e_1_4: f64,
    param_thetaParam: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    lda_xc_ksdt_lxc_pol_part0_zk_vrho_v2rho2(rho, zk, vrho, v2rho2, param_T, param_b_0_0, param_b_0_1, param_b_0_2, param_b_0_3, param_b_0_4, param_b_1_0, param_b_1_1, param_b_1_2, param_b_1_3, param_b_1_4, param_c_0_0, param_c_0_1, param_c_0_2, param_c_1_0, param_c_1_1, param_c_1_2, param_d_0_0, param_d_0_1, param_d_0_2, param_d_0_3, param_d_0_4, param_d_1_0, param_d_1_1, param_d_1_2, param_d_1_3, param_d_1_4, param_e_0_0, param_e_0_1, param_e_0_2, param_e_0_3, param_e_0_4, param_e_1_0, param_e_1_1, param_e_1_2, param_e_1_3, param_e_1_4, param_thetaParam, dens_threshold, zeta_threshold);
    lda_xc_ksdt_lxc_pol_part1_v3rho3_0(rho, v3rho3, param_T, param_b_0_0, param_b_0_1, param_b_0_2, param_b_0_3, param_b_0_4, param_b_1_0, param_b_1_1, param_b_1_2, param_b_1_3, param_b_1_4, param_c_0_0, param_c_0_1, param_c_0_2, param_c_1_0, param_c_1_1, param_c_1_2, param_d_0_0, param_d_0_1, param_d_0_2, param_d_0_3, param_d_0_4, param_d_1_0, param_d_1_1, param_d_1_2, param_d_1_3, param_d_1_4, param_e_0_0, param_e_0_1, param_e_0_2, param_e_0_3, param_e_0_4, param_e_1_0, param_e_1_1, param_e_1_2, param_e_1_3, param_e_1_4, param_thetaParam, dens_threshold, zeta_threshold);
    lda_xc_ksdt_lxc_pol_part2_v3rho3_1(rho, v3rho3, param_T, param_b_0_0, param_b_0_1, param_b_0_2, param_b_0_3, param_b_0_4, param_b_1_0, param_b_1_1, param_b_1_2, param_b_1_3, param_b_1_4, param_c_0_0, param_c_0_1, param_c_0_2, param_c_1_0, param_c_1_1, param_c_1_2, param_d_0_0, param_d_0_1, param_d_0_2, param_d_0_3, param_d_0_4, param_d_1_0, param_d_1_1, param_d_1_2, param_d_1_3, param_d_1_4, param_e_0_0, param_e_0_1, param_e_0_2, param_e_0_3, param_e_0_4, param_e_1_0, param_e_1_1, param_e_1_2, param_e_1_3, param_e_1_4, param_thetaParam, dens_threshold, zeta_threshold);
    lda_xc_ksdt_lxc_pol_part3_v3rho3_2(rho, v3rho3, param_T, param_b_0_0, param_b_0_1, param_b_0_2, param_b_0_3, param_b_0_4, param_b_1_0, param_b_1_1, param_b_1_2, param_b_1_3, param_b_1_4, param_c_0_0, param_c_0_1, param_c_0_2, param_c_1_0, param_c_1_1, param_c_1_2, param_d_0_0, param_d_0_1, param_d_0_2, param_d_0_3, param_d_0_4, param_d_1_0, param_d_1_1, param_d_1_2, param_d_1_3, param_d_1_4, param_e_0_0, param_e_0_1, param_e_0_2, param_e_0_3, param_e_0_4, param_e_1_0, param_e_1_1, param_e_1_2, param_e_1_3, param_e_1_4, param_thetaParam, dens_threshold, zeta_threshold);
    lda_xc_ksdt_lxc_pol_part4_v3rho3_3(rho, v3rho3, param_T, param_b_0_0, param_b_0_1, param_b_0_2, param_b_0_3, param_b_0_4, param_b_1_0, param_b_1_1, param_b_1_2, param_b_1_3, param_b_1_4, param_c_0_0, param_c_0_1, param_c_0_2, param_c_1_0, param_c_1_1, param_c_1_2, param_d_0_0, param_d_0_1, param_d_0_2, param_d_0_3, param_d_0_4, param_d_1_0, param_d_1_1, param_d_1_2, param_d_1_3, param_d_1_4, param_e_0_0, param_e_0_1, param_e_0_2, param_e_0_3, param_e_0_4, param_e_1_0, param_e_1_1, param_e_1_2, param_e_1_3, param_e_1_4, param_thetaParam, dens_threshold, zeta_threshold);
    lda_xc_ksdt_lxc_pol_part5_v4rho4_0(rho, v4rho4, param_T, param_b_0_0, param_b_0_1, param_b_0_2, param_b_0_3, param_b_0_4, param_b_1_0, param_b_1_1, param_b_1_2, param_b_1_3, param_b_1_4, param_c_0_0, param_c_0_1, param_c_0_2, param_c_1_0, param_c_1_1, param_c_1_2, param_d_0_0, param_d_0_1, param_d_0_2, param_d_0_3, param_d_0_4, param_d_1_0, param_d_1_1, param_d_1_2, param_d_1_3, param_d_1_4, param_e_0_0, param_e_0_1, param_e_0_2, param_e_0_3, param_e_0_4, param_e_1_0, param_e_1_1, param_e_1_2, param_e_1_3, param_e_1_4, param_thetaParam, dens_threshold, zeta_threshold);
    lda_xc_ksdt_lxc_pol_part6_v4rho4_1(rho, v4rho4, param_T, param_b_0_0, param_b_0_1, param_b_0_2, param_b_0_3, param_b_0_4, param_b_1_0, param_b_1_1, param_b_1_2, param_b_1_3, param_b_1_4, param_c_0_0, param_c_0_1, param_c_0_2, param_c_1_0, param_c_1_1, param_c_1_2, param_d_0_0, param_d_0_1, param_d_0_2, param_d_0_3, param_d_0_4, param_d_1_0, param_d_1_1, param_d_1_2, param_d_1_3, param_d_1_4, param_e_0_0, param_e_0_1, param_e_0_2, param_e_0_3, param_e_0_4, param_e_1_0, param_e_1_1, param_e_1_2, param_e_1_3, param_e_1_4, param_thetaParam, dens_threshold, zeta_threshold);
    lda_xc_ksdt_lxc_pol_part7_v4rho4_2(rho, v4rho4, param_T, param_b_0_0, param_b_0_1, param_b_0_2, param_b_0_3, param_b_0_4, param_b_1_0, param_b_1_1, param_b_1_2, param_b_1_3, param_b_1_4, param_c_0_0, param_c_0_1, param_c_0_2, param_c_1_0, param_c_1_1, param_c_1_2, param_d_0_0, param_d_0_1, param_d_0_2, param_d_0_3, param_d_0_4, param_d_1_0, param_d_1_1, param_d_1_2, param_d_1_3, param_d_1_4, param_e_0_0, param_e_0_1, param_e_0_2, param_e_0_3, param_e_0_4, param_e_1_0, param_e_1_1, param_e_1_2, param_e_1_3, param_e_1_4, param_thetaParam, dens_threshold, zeta_threshold);
    lda_xc_ksdt_lxc_pol_part8_v4rho4_3(rho, v4rho4, param_T, param_b_0_0, param_b_0_1, param_b_0_2, param_b_0_3, param_b_0_4, param_b_1_0, param_b_1_1, param_b_1_2, param_b_1_3, param_b_1_4, param_c_0_0, param_c_0_1, param_c_0_2, param_c_1_0, param_c_1_1, param_c_1_2, param_d_0_0, param_d_0_1, param_d_0_2, param_d_0_3, param_d_0_4, param_d_1_0, param_d_1_1, param_d_1_2, param_d_1_3, param_d_1_4, param_e_0_0, param_e_0_1, param_e_0_2, param_e_0_3, param_e_0_4, param_e_1_0, param_e_1_1, param_e_1_2, param_e_1_3, param_e_1_4, param_thetaParam, dens_threshold, zeta_threshold);
    lda_xc_ksdt_lxc_pol_part9_v4rho4_4(rho, v4rho4, param_T, param_b_0_0, param_b_0_1, param_b_0_2, param_b_0_3, param_b_0_4, param_b_1_0, param_b_1_1, param_b_1_2, param_b_1_3, param_b_1_4, param_c_0_0, param_c_0_1, param_c_0_2, param_c_1_0, param_c_1_1, param_c_1_2, param_d_0_0, param_d_0_1, param_d_0_2, param_d_0_3, param_d_0_4, param_d_1_0, param_d_1_1, param_d_1_2, param_d_1_3, param_d_1_4, param_e_0_0, param_e_0_1, param_e_0_2, param_e_0_3, param_e_0_4, param_e_1_0, param_e_1_1, param_e_1_2, param_e_1_3, param_e_1_4, param_thetaParam, dens_threshold, zeta_threshold);
}
