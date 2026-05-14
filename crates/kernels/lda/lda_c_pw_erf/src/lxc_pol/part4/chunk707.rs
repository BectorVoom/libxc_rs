//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 707/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk707<F: Float>(t817: F, t925: F, t1945: F, t325: F, t1950: F, t1955: F, t4606: F, t3589: F, t743: F, t951: F) -> (F, F, F, F, F, F) {
    let t4657 = t925 * t817;
    let t4659 = t325 * t1945;
    let t4661 = t325 * t1950;
    let t4662 = 0.002518888888888889 * t4661;
    let t4663 = t4606 * t1955;
    let t4665 = t3589 * t743;
    let t4666 = t4665 * t951;
    (t4657, t4659, t4661, t4662, t4663, t4666)
}
