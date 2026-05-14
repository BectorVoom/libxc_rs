//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1106/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1106<F: Float>(t16537: F, t16505: F, t16507: F, t16510: F, t16512: F, t16516: F, t16518: F, t16521: F, t16523: F, t16525: F, t16530: F, t16531: F, t16534: F, t16536: F, t5120: F, t802: F) -> (F, F, F) {
    let t16538 = 4.0 / 45.0 * t16537;
    let t16539 = -t16505 + t16507 - t16510 - t16512 - t16516 - t16518 - t16521 + t16523 + t16525 - t16530 - t16531 + t16534 - t16536 - t16538;
    let t16541 = 2.0 / 15.0 * t802 * t5120;
    (t16538, t16539, t16541)
}
