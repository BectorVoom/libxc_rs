//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1045/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1045<F: Float>(t12587: F, t12591: F, t12597: F, t12602: F, t12604: F, t12608: F, t12610: F, t12612: F, t12620: F, t12622: F, t12624: F, t12626: F, t12628: F, t12630: F, t12632: F, t12636: F, t12641: F, t12643: F, t12645: F, t12648: F, t12650: F, t12653: F, t12654: F) -> (F, F) {
    let t14380 = t12587 - t12591 + t12597 - t12602 - t12604 - t12608 + t12610 - t12612 - t12620 + t12622 - t12624;
    let t14381 = -t12626 - t12628 - t12630 - t12632 + t12636 - t12641 - t12643 - t12645 - t12648 - t12650 - t12653 - t12654;
    (t14380, t14381)
}
