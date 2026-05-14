//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 600/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk600<F: Float>(t1442: F, t3787: F, t1325: F, t3726: F, t3729: F, t3734: F, t3737: F, t3741: F, t3744: F, t3747: F, t3750: F, t3754: F, t3759: F, t3761: F, t3765: F, t3767: F, t3772: F, t3777: F, t3782: F, t3786: F) -> (F, F, F, F) {
    let t3788 = t3787 * t1442;
    let t3789 = t1325 * t3788;
    let t3790 = 16.0 / 15.0 * t3789;
    let t3791 = -t3726 + t3729 + t3734 - t3737 + t3741 + t3744 + t3747 - t3750 + t3754 + t3759 + t3761 - t3765 - t3767 - t3772 - t3777 + t3782 - t3786 - t3790;
    (t3788, t3789, t3790, t3791)
}
