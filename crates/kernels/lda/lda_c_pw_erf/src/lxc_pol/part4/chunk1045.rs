//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1045/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1045<F: Float>(t117: F, t174: F, t14654: F, t8896: F, t127: F, t3296: F, t14666: F, t431: F, t5571: F, t5509: F, t925: F, t2061: F, t5512: F, t14646: F, t5592: F, t14639: F, t1686: F, t1852: F) -> (F, F, F, F, F, F, F, F) {
    let t14777 = t117 * t174;
    let t14781 = t8896 * t14654;
    let t14783 = t127 * t3296;
    let t14787 = t431 * t5571 * t14666;
    let t14795 = t5509 * t925;
    let t14797 = t5512 * t2061;
    let t14799 = t5592 * t14646;
    let t14802 = t1686 * t1852 * t14639;
    (t14777, t14781, t14783, t14787, t14795, t14797, t14799, t14802)
}
