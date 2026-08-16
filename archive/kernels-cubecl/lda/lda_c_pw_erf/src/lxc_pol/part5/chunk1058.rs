//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1058/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1058<F: Float>(t14679: F, t1870: F, t7195: F, t5639: F, t7199: F, t133: F, t19539: F, t19532: F, t19551: F, t101: F, t776: F, t9134: F) -> (F, F, F, F, F, F) {
    let t19750 = t1870 * t14679 * t7195;
    let t19753 = t1870 * t5639 * t7199;
    let t19773 = t133 * t19539;
    let t19775 = t133 * t19532;
    let t19782 = t133 * t19551;
    let t19832 = t101 * t776 * t9134;
    (t19750, t19753, t19773, t19775, t19782, t19832)
}
