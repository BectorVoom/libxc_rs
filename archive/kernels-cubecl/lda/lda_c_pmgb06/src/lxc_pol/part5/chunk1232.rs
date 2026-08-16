//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1232/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1232<F: Float>(t12829: F, t12832: F, t20205: F, t20207: F, t20209: F, t20210: F, t20211: F, t20213: F, t20215: F, t9759: F, t9770: F, t20219: F, t20221: F, t20222: F, t20224: F, t20226: F, t20235: F, t20238: F, t20241: F, t20243: F, t20247: F, t20250: F, t20253: F) -> (F, F) {
    let t21978 = -t20205 + t9759 - t20207 - t12829 - t12832 + t20209 + t20210 - t20211 - t9770 - t20213 + t20215;
    let t21979 = t20219 - t20221 - t20222 + t20224 + t20226 - t20235 - t20238 + t20241 - t20243 - t20247 - t20250 + t20253;
    (t21978, t21979)
}
