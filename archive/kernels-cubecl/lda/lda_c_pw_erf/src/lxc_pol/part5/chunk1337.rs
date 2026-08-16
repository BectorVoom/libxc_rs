//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1337/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1337<F: Float>(t10409: F, t10412: F, t10414: F, t22084: F, t22086: F, t22088: F, t22093: F, t22098: F, t22102: F, t22107: F, t22109: F, t22111: F, t22113: F) -> F {
    let t23289 = -t22084 + t22086 + t22088 + t22093 - t22098 + t22102 - t22107 - t22109 + t22111 - t22113 + F::cast_from(4.0_f64) * t10409 + t10412 + F::cast_from(0.0011033703703703704_f64) * t10414;
    t23289
}
