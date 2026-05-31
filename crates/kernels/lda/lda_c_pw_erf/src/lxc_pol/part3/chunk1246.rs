//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1246/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1246<F: Float>(t14639: F, t1686: F, t1852: F, t10: F, t14634: F, t14656: F, t14658: F, t14660: F, t14781: F, t14783: F, t14787: F, t14796: F, t14797: F, t14799: F, t3222: F, t426: F, t5571: F) -> F {
    let t14802 = t1686 * t1852 * t14639;
    let t14803 = F::cast_from(5.87616_f64) * t14802;
    let t14804 = -F::cast_from(0.97936_f64) * t14781 - F::cast_from(88.1424_f64) * t14783 * t14658 + t14656 - t14660 + F::cast_from(44.0712_f64) * t14787 + F::cast_from(30.0_f64) * t426 * t10 * t5571 * t3222 - t426 * t14634 / F::cast_from(2.0_f64) - t14796 + F::cast_from(1.95872_f64) * t14797 - F::cast_from(8.81424_f64) * t14799 + t14803;
    t14804
}
