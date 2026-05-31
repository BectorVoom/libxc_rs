//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1130/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1130<F: Float>(t15963: F, t15966: F, t13202: F, t34: F, t519: F, t6426: F, t4738: F, t6469: F, t11947: F, t11955: F, t20921: F, t20923: F, t20925: F, t20927: F, t20929: F, t20931: F, t20932: F, t20933: F) -> (F, F, F, F, F) {
    let t20934 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t15963;
    let t20935 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t15966;
    let t20939 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t519 * t13202 * t6426 * t34;
    let t20941 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t4738 * t6469;
    let t20942 = -t11947 + t20921 - t20923 - t20925 + t20927 + t20929 + t20931 - t11955 + t20932 - t20933 + t20934 - t20935 + t20939 - t20941;
    (t20934, t20935, t20939, t20941, t20942)
}
