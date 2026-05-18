//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1211/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1211<F: Float>(t13214: F, t13216: F, t13219: F, t13221: F, t13223: F, t13225: F, t13226: F, t13227: F, t13231: F, t13233: F, t13236: F, t13238: F, t13240: F, t13242: F, t13244: F, t13246: F, t13248: F, t13250: F, t13252: F, t13257: F, t13258: F, t13260: F, t13262: F) -> (F, F) {
    let t14417 = t13214 + t13216 - t13219 + t13221 - t13223 - t13225 - t13226 + t13227 + t13231 + t13233 + t13236;
    let t14418 = t13238 + t13240 + t13242 - t13244 - t13246 - t13248 - t13250 - t13252 - t13257 - t13258 + t13260 + t13262;
    (t14417, t14418)
}
