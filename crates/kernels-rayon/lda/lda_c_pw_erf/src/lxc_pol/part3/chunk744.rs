//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 744/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk744(t1322: f64, t4763: f64, t1472: f64, t2023: f64, t2065: f64, t558: f64, t352: f64, t1308: f64, t571: f64, t2017: f64, t4680: f64, t219: f64, t4049: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4765 = 16.0_f64 / 45.0_f64 * t4763 * t1322;
    let t4767 = 8.0_f64 / 45.0_f64 * t1472 * t2023;
    let t4768 = t2065 * t558;
    let t4769 = t4768 * t352;
    let t4770 = t1308 * t4769;
    let t4772 = 8.0_f64 / 45.0_f64 * t571 * t4770;
    let t4773 = t2017 * t4680;
    let t4775 = 4.0_f64 / 27.0_f64 * t571 * t4773;
    let t4776 = t4049 * t219;
    (t4765, t4767, t4768, t4769, t4770, t4772, t4773, t4775, t4776)
}
