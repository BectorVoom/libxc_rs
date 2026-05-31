//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 983/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk983<F: Float>(t8131: F, t10500: F, t10505: F, t10509: F, t10511: F, t10515: F, t10518: F, t10520: F, t10522: F, t10525: F, t10528: F, t10531: F, t10533: F) -> F {
    let t11694 = F::cast_from(48.0_f64) * t8131;
    let t11695 = t10500 + t10505 + t10509 - t10511 + t10515 - t10518 - t10520 + t10522 + t10525 + t10528 - t10531 - t11694 + t10533;
    t11695
}
