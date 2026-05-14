//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 778/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk778<F: Float>(t1697: F, t7918: F, t10: F, t3280: F, t3282: F, t3284: F, t3288: F, t3290: F, t426: F, t5502: F, t5507: F, t5513: F, t7893: F, t7896: F, t7897: F, t7915: F) -> (F, F, F) {
    let t7919 = t1697 * t7918;
    let t7920 = t10 * t7919;
    let t7923 = -2.93808 * t5502 - t7893 - 2.0 / 3.0 * t5507 - 1.46904 * t5513 + t7896 + t3280 - t3282 - t3284 - t3288 - t3290 + 9.0 / 2.0 * t426 * t10 * t7897 - t426 * t7915 / 2.0 - 6.0 * t426 * t7920;
    (t7919, t7920, t7923)
}
