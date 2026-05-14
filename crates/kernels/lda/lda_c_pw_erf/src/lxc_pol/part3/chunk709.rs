//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 709/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk709<F: Float>(t184: F, t5044: F, t813: F, t1280: F, t795: F, t4073: F, t1508: F, t808: F, t1234: F, t1294: F, t822: F, t1302: F, t2120: F, t3455: F, t786: F, t4966: F, t4968: F, t4970: F, t4972: F, t5033: F, t5035: F, t5037: F, t5039: F, t5043: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5045 = t5044 * t184;
    let t5047 = 8.0 / 15.0 * t5045 * t813;
    let t5049 = 2.0 / 15.0 * t795 * t1280;
    let t5051 = 4.0 / 15.0 * t4073 * t813;
    let t5053 = 2.0 / 15.0 * t1508 * t808;
    let t5055 = 8.0 / 45.0 * t795 * t1234;
    let t5057 = 8.0 / 45.0 * t822 * t1294;
    let t5059 = 4.0 / 15.0 * t2120 * t1302;
    let t5061 = 4.0 / 15.0 * t3455 * t786;
    let t5062 = -t4966 - t4968 - t4970 - t4972 - t5033 - t5035 - t5037 + t5039 + t5043 + t5047 - t5049 + t5051 - t5053 - t5055 - t5057 + t5059 + t5061;
    (t5045, t5047, t5049, t5051, t5053, t5055, t5057, t5059, t5061, t5062)
}
