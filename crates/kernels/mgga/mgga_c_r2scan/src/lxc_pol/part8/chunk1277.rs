//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1277/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1277<F: Float>(t3073: F, t6240: F, t1632: F, t551: F, t6218: F, t9439: F, t5136: F, t9337: F, t3071: F, t481: F, t1568: F, t6155: F, t29726: F, t7623: F, t6205: F, t9177: F) -> (F, F, F, F, F, F, F) {
    let t29810 = t6240 * t3073;
    let t29814 = t6218 * t551 * t1632 * t9439;
    let t29822 = t5136 * t551 * t1632 * t9337;
    let t29837 = t3071 * t481;
    let t29839 = t6155 * t1568 * t29837;
    let t29842 = t7623 * t1568 * t29726;
    let t29851 = t6205 * t9177;
    (t29810, t29814, t29822, t29837, t29839, t29842, t29851)
}
