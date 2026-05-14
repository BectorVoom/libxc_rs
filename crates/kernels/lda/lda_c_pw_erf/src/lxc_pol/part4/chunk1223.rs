//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1223/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1223<F: Float>(t1466: F, t2161: F, t5029: F, t571: F, t1381: F, t6193: F, t518: F, t6630: F, t525: F, t1325: F, t3787: F, t6916: F, t4738: F, t5394: F, t6980: F, t1278: F, t1440: F, t6979: F) -> (F, F, F, F, F, F, F) {
    let t18149 = 8.0 / 15.0 * t571 * t1466 * t2161 * t5029;
    let t18153 = 4.0 / 15.0 * t571 * t1466 * t6193 * t1381;
    let t18154 = t6630 * t518;
    let t18156 = 8.0 / 45.0 * t18154 * t525;
    let t18158 = t1325 * t3787 * t6916;
    let t18159 = 32.0 / 45.0 * t18158;
    let t18161 = 8.0 / 15.0 * t4738 * t5394;
    let t18163 = t1325 * t3787 * t6980;
    let t18164 = 16.0 / 45.0 * t18163;
    let t18168 = 4.0 / 15.0 * t1325 * t1440 * t6979 * t1278;
    (t18149, t18153, t18156, t18159, t18161, t18164, t18168)
}
