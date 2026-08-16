//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 742/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk742<F: Float>(t4494: F, t6716: F, t4501: F, t743: F, t833: F, t3976: F, t549: F, t4507: F, t558: F, t593: F, t352: F, t4515: F) -> (F, F, F, F, F, F, F, F) {
    let t6717 = t4494 * t6716;
    let t6720 = t4501 * t6716;
    let t6723 = t743 * t833;
    let t6725 = t3976 * t6723 * t549;
    let t6728 = t4507 * t558;
    let t6730 = t6728 * t6723 * t593;
    let t6733 = t6723 * t352;
    let t6734 = t4515 * t6733;
    (t6717, t6720, t6723, t6725, t6728, t6730, t6733, t6734)
}
