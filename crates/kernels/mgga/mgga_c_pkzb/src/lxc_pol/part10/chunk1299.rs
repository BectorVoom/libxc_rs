//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1299/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1299<F: Float>(t1976: F, t1979: F, t25597: F, t730: F, t3622: F, t5754: F, t2860: F, t7223: F, t1987: F, t9205: F, t7312: F, t7411: F, t2759: F, t1861: F, t17351: F, t17354: F, t17455: F, t20705: F, t20716: F, t20719: F, t25633: F, t25636: F, t25639: F) -> (F, F, F, F, F, F, F, F) {
    let t25695 = 0.34631718211362927518e2 * t730 * t1976 * t25597 * t1979;
    let t25697 = 0.5848223622634646207e0 * t5754 * t3622;
    let t25699 = 0.34631718211362927517e2 * t2860 * t7223;
    let t25701 = 0.11696447245269292414e1 * t1987 * t9205;
    let t25703 = 12.0 * t7411 * t7312;
    let t25704 = t2759 * t2759;
    let t25705 = t1861 * t25704;
    let t25714 = t17455 - 56.0 / 27.0 * t17351 + 4.0 / 9.0 * t17354 - 56.0 / 27.0 * t20705 + 16.0 / 9.0 * t20716 - 2.0 / 3.0 * t20719 + 4.0 / 9.0 * t25633 - 2.0 / 3.0 * t25636 + t25639;
    (t25695, t25697, t25699, t25701, t25703, t25704, t25705, t25714)
}
