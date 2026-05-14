//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 954/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk954<F: Float>(t11466: F, t11472: F, t14423: F, t20081: F, t20082: F, t20084: F, t20085: F, t20086: F, t8414: F, t8417: F, t8419: F, t8423: F, t8427: F, t8432: F, t8437: F, t8445: F, t8449: F) -> (F,) {
    let t20196 = t11466 + t8414 + t8417 + t8419 + t20081 - t20082 + t11472 + t8423 - t8427 + t8432 + t8437 - t20084 + t8445 - t8449 - t14423 + t20085 + t20086;
    (t20196,)
}
