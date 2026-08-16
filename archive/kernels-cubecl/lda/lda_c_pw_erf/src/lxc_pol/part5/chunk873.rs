//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 873/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk873<F: Float>(t174: F, t2: F, t2824: F, t8138: F, t2716: F, t343: F, t1191: F, t732: F, t918: F, t1184: F, t119: F, t321: F) -> (F, F, F, F, F, F) {
    let t8141 = t8138 * t2 * t2824 * t174;
    let t8143 = t2716 * t343;
    let t8145 = t732 * t1191;
    let t8146 = t918 * t8145;
    let t8148 = t119 * t1184;
    let t8149 = t321 * t8148;
    (t8141, t8143, t8145, t8146, t8148, t8149)
}
