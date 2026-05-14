//! LDA_C_PK09 fxc pol kernel — fxc_pol (nested-by-output, 4 parts).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod part0;
mod part1;
mod part2;
mod part3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

use part0::lda_c_pk09_fxc_pol_part0_zk_vrho;
use part1::lda_c_pk09_fxc_pol_part1_v2rho2_0;
use part2::lda_c_pk09_fxc_pol_part2_v2rho2_1;
use part3::lda_c_pk09_fxc_pol_part3_v2rho2_2;

/// LDA_C_PK09 fxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_c_pk09_fxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    lda_c_pk09_fxc_pol_part0_zk_vrho(rho, zk, vrho, dens_threshold, zeta_threshold);
    lda_c_pk09_fxc_pol_part1_v2rho2_0(rho, v2rho2, dens_threshold, zeta_threshold);
    lda_c_pk09_fxc_pol_part2_v2rho2_1(rho, v2rho2, dens_threshold, zeta_threshold);
    lda_c_pk09_fxc_pol_part3_v2rho2_2(rho, v2rho2, dens_threshold, zeta_threshold);
}
