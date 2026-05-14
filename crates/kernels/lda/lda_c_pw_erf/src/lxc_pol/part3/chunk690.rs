//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 690/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk690<F: Float>(t1322: F, t4763: F, t1472: F, t2023: F, t2065: F, t558: F, t352: F, t1308: F, t571: F, t2017: F, t4680: F, t219: F, t4049: F, t4666: F, t1287: F, t816: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4765 = 16.0 / 45.0 * t4763 * t1322;
    let t4767 = 8.0 / 45.0 * t1472 * t2023;
    let t4768 = t2065 * t558;
    let t4769 = t4768 * t352;
    let t4770 = t1308 * t4769;
    let t4772 = 8.0 / 45.0 * t571 * t4770;
    let t4773 = t2017 * t4680;
    let t4775 = 4.0 / 27.0 * t571 * t4773;
    let t4776 = t4049 * t219;
    let t4777 = t4776 * t4666;
    let t4779 = 32.0 / 81.0 * t571 * t4777;
    let t4780 = t816 * t1287;
    (t4765, t4767, t4768, t4769, t4770, t4772, t4773, t4775, t4776, t4777, t4779, t4780)
}
