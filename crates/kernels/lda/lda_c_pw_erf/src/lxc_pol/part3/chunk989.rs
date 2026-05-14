//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 989/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk989<F: Float>(t13217: F, t1325: F, t1991: F, t2954: F, t4804: F, t5418: F, t3794: F, t1976: F, t4829: F, t945: F, t11766: F, t5256: F, t1446: F, t5421: F, t13201: F, t13206: F, t13208: F, t13210: F, t13212: F, t13214: F, t13216: F) -> (F, F, F, F, F, F, F) {
    let t13221 = 16.0 / 9.0 * t1325 * t1991 * t13217 * t2954;
    let t13223 = 16.0 / 15.0 * t4804 * t5418;
    let t13225 = 16.0 / 15.0 * t3794 * t5418;
    let t13229 = 8.0 / 15.0 * t1325 * t4829 * t1976 * t945;
    let t13232 = 8.0 / 9.0 * t1325 * t5256 * t11766;
    let t13233 = t1446 * t5421;
    let t13234 = 16.0 / 45.0 * t13233;
    let t13235 = t13201 + t13206 + t13208 + t13210 + t13212 - t13214 - t13216 - t13221 + t13223 + t13225 + t13229 + t13232 - t13234;
    (t13221, t13223, t13225, t13229, t13232, t13234, t13235)
}
