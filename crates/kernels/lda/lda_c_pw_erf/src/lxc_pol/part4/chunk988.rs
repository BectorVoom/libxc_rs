//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 988/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk988<F: Float>(t169: F, t2817: F, t301: F, t865: F, t164: F, t4422: F, t145: F, t4713: F, t479: F, t5446: F, t1590: F, t1901: F, t1896: F, t11546: F, t4263: F, t781: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11597 = t169 * t2817 * t865 * t301;
    let t11620 = t4422 * t164;
    let t11622 = t145 * t4713;
    let t11623 = t11622 * t164;
    let t11625 = t5446 * t479;
    let t11627 = t1901 * t1590;
    let t11629 = t1896 * t479;
    let t11631 = t11546 * t164;
    let t11633 = t781 * t4263;
    (t11597, t11620, t11622, t11623, t11625, t11627, t11629, t11631, t11633)
}
