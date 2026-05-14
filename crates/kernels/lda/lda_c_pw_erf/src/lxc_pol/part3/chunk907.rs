//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 907/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk907<F: Float>(t1302: F, t5215: F, t11906: F, t11912: F, t11917: F, t11918: F, t11919: F, t11920: F, t11921: F, t11925: F, t11927: F, t11929: F, t9250: F, t9253: F, t5401: F, t568: F) -> (F, F, F) {
    let t11931 = 4.0 / 5.0 * t5215 * t1302;
    let t11933 = t11906 - t11912 + t11917 - t11918 - t11919 + t11920 - t9250 - t11921 + t11925 + t11927 + t11929 + t11931 + 4.0 * t9253;
    let t11936 = t5401 * t568;
    (t11931, t11933, t11936)
}
