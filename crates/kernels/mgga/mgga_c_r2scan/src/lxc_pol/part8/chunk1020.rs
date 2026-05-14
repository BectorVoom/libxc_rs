//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1020/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1020<F: Float>(t552: F, t9981: F, t551: F, t9937: F, t2567: F, t3056: F, t360: F, t8778: F, t921: F, t8783: F, t2124: F, t8842: F, t113: F, t910: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9982 = t552 * t9981;
    let t9983 = t551 * t9982;
    let t9986 = t552 * t9937;
    let t9987 = t551 * t9986;
    let t9990 = t2567 * t3056;
    let t9991 = t360 * t9990;
    let t9994 = t8778 * t921;
    let t9995 = t360 * t9994;
    let t9998 = t8783 * t921;
    let t9999 = t360 * t9998;
    let t10007 = t2124 * t8842 * t921;
    let t10010 = t113 * t910;
    (t9983, t9987, t9990, t9991, t9994, t9995, t9998, t9999, t10007, t10010)
}
