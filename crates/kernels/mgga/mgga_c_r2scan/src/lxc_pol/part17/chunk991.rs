//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 991/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk991<F: Float>(t1575: F, t269: F, t546: F, t565: F, t10728: F, t7258: F, t39960: F, t10729: F, t11659: F, t6395: F, t10868: F, t7614: F, t7615: F, t39885: F, t8243: F, t2605: F, t37699: F) -> (F, F, F, F, F, F, F, F, F) {
    let t40061 = t1575 * t269;
    let t40062 = t546 * t40061;
    let t40066 = t565 * t40061;
    let t40070 = t10728 * t7258;
    let t40075 = t546 * t39960;
    let t40076 = t40075 * t10729;
    let t40086 = t6395 * t11659;
    let t40090 = t7614 * t10868 * t7615;
    let t40102 = t39885 * t8243;
    let t40107 = t37699 * t2605;
    (t40062, t40066, t40070, t40075, t40076, t40086, t40090, t40102, t40107)
}
