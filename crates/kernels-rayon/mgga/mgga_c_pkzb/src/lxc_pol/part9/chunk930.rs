//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 930/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk930(t6902: f64, t6937: f64, t6969: f64, t7094: f64, t158: f64, t1054: f64, t1791: f64, t5418: f64, t1790: f64, t2702: f64, t633: f64, t1812: f64, t2678: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7096 = t6902 + t6937 + t6969 + t7094;
    let t7097 = t7096 * t158;
    let t7113 = t5418 * t1054 * t1791;
    let t7116 = t1790 * t2702;
    let t7117 = t7116 * t633;
    let t7120 = t2678 * t1812;
    (t7096, t7097, t7113, t7116, t7117, t7120)
}
