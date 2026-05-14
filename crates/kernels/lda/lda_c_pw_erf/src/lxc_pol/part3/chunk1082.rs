//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1082/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1082<F: Float>(t14666: F, t431: F, t5571: F, t5509: F, t925: F, t2061: F, t5512: F, t14646: F, t5592: F, t14639: F, t1686: F, t1852: F, t10: F, t14634: F, t14656: F, t14658: F, t14660: F, t14781: F, t14783: F, t3222: F, t426: F) -> (F,) {
    let t14787 = t431 * t5571 * t14666;
    let t14795 = t5509 * t925;
    let t14796 = 2.93808 * t14795;
    let t14797 = t5512 * t2061;
    let t14799 = t5592 * t14646;
    let t14802 = t1686 * t1852 * t14639;
    let t14803 = 5.87616 * t14802;
    let t14804 = -0.97936 * t14781 - 88.1424 * t14783 * t14658 + t14656 - t14660 + 44.0712 * t14787 + 30.0 * t426 * t10 * t5571 * t3222 - t426 * t14634 / 2.0 - t14796 + 1.95872 * t14797 - 8.81424 * t14799 + t14803;
    (t14804,)
}
