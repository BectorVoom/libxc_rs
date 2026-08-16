//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1113/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1113<F: Float>(t13205: F, t13207: F, t13210: F, t13212: F, t13214: F, t13216: F, t13219: F, t13221: F, t13223: F, t13225: F, t13226: F, t13227: F) -> F {
    let t13228 = t13205 + t13207 + t13210 + t13212 + t13214 + t13216 - t13219 + t13221 - t13223 - t13225 - t13226 + t13227;
    t13228
}
