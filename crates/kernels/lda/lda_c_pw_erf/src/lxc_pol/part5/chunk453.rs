//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 453/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk453<F: Float>(t299: F, t780: F, t169: F, t242: F, t171: F, t1904: F, t632: F, t875: F, t462: F, t865: F, t1101: F, t1104: F, t1108: F, t1118: F, t1146: F, t1148: F, t1149: F, t145: F, t1891: F) -> (F, F, F, F, F, F) {
    let t2220 = t299 * t780;
    let t2222 = t169 * t2220 * t242;
    let t2224 = t171 * t1904;
    let t2229 = t169 * t875 * t632;
    let t2233 = t462 * t865;
    let t2237 = -t1101 + 0.053059442957798957 * t1104 + t1108 + 0.053059442957798957 * t2222 - 0.031835665774679375 * t169 * t2224 * t242 - 0.031835665774679375 * t2229 - 0.031835665774679375 * t1118 - t1146 + t1148 - 0.10665013548435875 * t1149 - 0.10665013548435875 * t2233 + 0.05332506774217938 * t145 * t1891;
    (t2220, t2222, t2224, t2229, t2233, t2237)
}
