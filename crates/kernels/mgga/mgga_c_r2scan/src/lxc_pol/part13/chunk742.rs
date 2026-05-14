//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 742/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk742<F: Float>(t5021: F, t5872: F, t5874: F, t5871: F, t5878: F, t1509: F, t898: F, t41: F, t1531: F, t2463: F, t2: F, t2483: F, t464: F, t2266: F, t6599: F, t910: F) -> (F, F, F, F, F, F) {
    let t7025 = 4.0 * t5021;
    let t7026 = 1584.0 * t5872;
    let t7027 = 1872.0 * t5874;
    let t7028 = t5871 - t7026 - t7027 + t5878;
    let t7030 = t898 * t1509;
    let t7031 = t41 * t7030;
    let t7032 = t2463 * t1531;
    let t7033 = 0.24415263074675393405e-3 * t7032;
    let t7034 = t2483 * t2;
    let t7035 = t7034 * t464;
    let t7036 = 0.36622894612013090108e-3 * t7035;
    let t7038 = t2266 * t6599 * t910;
    (t7025, t7028, t7031, t7033, t7036, t7038)
}
