//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 903/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk903<F: Float>(t409: F, t419: F, t421: F, t7364: F, t117: F, t123: F, t315: F, t7869: F, t10670: F, t14275: F, t14279: F, t14284: F, t14291: F, t14293: F, t14298: F, t14300: F, t14303: F, t14306: F, t14308: F, t15152: F, t15159: F) -> (F,) {
    let t19109 = t409 * t7364 * t419 * t421;
    let t19118 = t123 * t315 * t7869 * t117;
    let t19124 = 0.0878110494085338 * t10670 - 0.001975389032890948 * t19109 - 0.07769863529371063 * t14275 + 0.010403978958430045 * t14279 - t14284 - t14291 - 0.01777850129601853 * t14293 + t14298 + 0.059261670986728444 * t14300 - 0.004458848125041448 * t14303 + 0.008980675507690957 * t19118 - 0.07184540406152766 * t15152 + 0.5670973300165402 * t14306 - 0.00035595929614954216 * t14308 + 0.01975389032890948 * t15159;
    (t19124,)
}
