//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 770/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk770<F: Float>(t1302: F, t2120: F, t3455: F, t786: F, t4966: F, t4968: F, t4970: F, t4972: F, t5033: F, t5035: F, t5037: F, t5039: F, t5043: F, t5047: F, t5049: F, t5051: F, t5053: F, t5055: F, t5057: F) -> (F, F, F) {
    let t5059 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t2120 * t1302;
    let t5061 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t3455 * t786;
    let t5062 = -t4966 - t4968 - t4970 - t4972 - t5033 - t5035 - t5037 + t5039 + t5043 + t5047 - t5049 + t5051 - t5053 - t5055 - t5057 + t5059 + t5061;
    (t5059, t5061, t5062)
}
