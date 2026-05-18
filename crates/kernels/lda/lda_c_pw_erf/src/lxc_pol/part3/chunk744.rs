//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 744/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk744<F: Float>(t1322: F, t4763: F, t1472: F, t2023: F, t2065: F, t558: F, t352: F, t1308: F, t571: F, t2017: F, t4680: F, t219: F, t4049: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4765 = F::new(16.0) / F::new(45.0) * t4763 * t1322;
    let t4767 = F::new(8.0) / F::new(45.0) * t1472 * t2023;
    let t4768 = t2065 * t558;
    let t4769 = t4768 * t352;
    let t4770 = t1308 * t4769;
    let t4772 = F::new(8.0) / F::new(45.0) * t571 * t4770;
    let t4773 = t2017 * t4680;
    let t4775 = F::new(4.0) / F::new(27.0) * t571 * t4773;
    let t4776 = t4049 * t219;
    (t4765, t4767, t4768, t4769, t4770, t4772, t4773, t4775, t4776)
}
