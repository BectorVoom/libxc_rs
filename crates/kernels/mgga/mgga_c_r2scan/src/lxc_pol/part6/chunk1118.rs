//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1118/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1118<F: Float>(t20137: F, t6475: F, t6476: F, t1632: F, t5136: F, t5137: F, t551: F, t19786: F, t546: F, t19790: F, t560: F, t19789: F, t5052: F, t545: F, t548: F, t410: F, t4879: F) -> (F, F, F, F, F, F, F) {
    let t20139 = t6475 * t20137 * t6476;
    let t20143 = t5136 * t551 * t1632 * t5137;
    let t20145 = t546 * t19786;
    let t20146 = t19790 * t560;
    let t20148 = t20145 * t19789 * t20146;
    let t20150 = t545 * t5052;
    let t20151 = t20150 * t548;
    let t20180 = 16.0 * t410 * t4879;
    (t20139, t20143, t20145, t20148, t20150, t20151, t20180)
}
