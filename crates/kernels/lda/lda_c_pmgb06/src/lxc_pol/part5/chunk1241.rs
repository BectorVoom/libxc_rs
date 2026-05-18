//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1241/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1241<F: Float>(t10079: F, t10082: F, t13244: F, t20521: F, t20523: F, t20525: F, t20529: F, t20533: F, t20536: F, t20539: F, t20541: F, t20543: F) -> F {
    let t22005 = t20521 - t20523 - t20525 - t20529 - t13244 - F::new(8.0) / F::new(405.0) * t10079 + t10082 + t20533 + t20536 - t20539 + t20541 + t20543;
    t22005
}
