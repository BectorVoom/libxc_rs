//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 737/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk737<F: Float>(t163: F, t169: F, t717: F, t841: F, t164: F, t1896: F, t1590: F, t781: F, t145: F, t1904: F) -> (F, F, F, F) {
    let t5440 = t169 * t717 * t841 * t163;
    let t5442 = t1896 * t164;
    let t5444 = t781 * t1590;
    let t5446 = t145 * t1904;
    (t5440, t5442, t5444, t5446)
}
