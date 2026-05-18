//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1011/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1011<F: Float>(t331: F, t6812: F, t325: F, t6643: F, t6640: F, t1351: F, t6005: F, t6662: F, t6659: F, t1333: F, t4606: F, t6646: F) -> (F, F, F, F, F, F, F, F) {
    let t16287 = t331 * t6812;
    let t16292 = t325 * t6643;
    let t16297 = t325 * t6640;
    let t16305 = t1351 * t6005;
    let t16325 = t325 * t6662;
    let t16327 = t325 * t6659;
    let t16329 = t1333 * t6005;
    let t16338 = t4606 * t6646;
    (t16287, t16292, t16297, t16305, t16325, t16327, t16329, t16338)
}
