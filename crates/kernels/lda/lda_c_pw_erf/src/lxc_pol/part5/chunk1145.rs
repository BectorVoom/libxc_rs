//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1145/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1145<F: Float>(t22449: F, t22450: F, t22578: F, t22580: F, t22584: F, t22586: F, t22587: F, t22588: F, t22589: F, t22590: F, t22591: F, t22594: F, t22599: F, t22602: F, t22606: F, t22610: F, t22613: F, t22616: F, t22619: F, t22622: F, t22624: F, t22626: F, t22629: F, t22631: F, t22634: F, t22636: F) -> (F, F) {
    let t23312 = -t22449 + t22450 - t22578 + t22580 + t22584 + t22586 - t22587 - t22588 - t22589 + t22590 - t22591 - t22594 - t22599;
    let t23314 = -t22602 - t22606 + t22610 + t22613 - t22616 - t22619 - t22622 - t22624 - t22626 - t22629 - t22631 - t22634 - t22636;
    (t23312, t23314)
}
