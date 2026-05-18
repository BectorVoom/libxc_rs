//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1412/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1412<F: Float>(t16640: F, t16649: F, t16652: F, t16654: F, t16657: F, t16659: F, t16662: F, t16665: F, t16668: F, t16671: F, t16673: F, t16676: F, t16679: F, t16682: F, t16685: F) -> F {
    let t18255 = t16640 + t16649 + t16652 + t16654 + t16657 - t16659 - t16662 - t16665 - t16668 - t16671 + t16673 + t16676 + t16679 + t16682 + t16685;
    t18255
}
