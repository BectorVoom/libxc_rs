//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 511/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk511<F: Float>(t155: F, t2592: F, t802: F, t815: F, t824: F, t851: F) -> (F, F, F, F) {
    let t2594 = t2592 * t155 / F::new(30.0);
    let t2596 = t802 * t815 / F::new(15.0);
    let t2598 = t802 * t824 / F::new(15.0);
    let t2599 = t851 * t851;
    (t2594, t2596, t2598, t2599)
}
