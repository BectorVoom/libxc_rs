//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 832/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk832(t5626: f64, t5677: f64, t5924: f64, t5987: f64, t158: f64, t789: f64, t2119: f64, t799: f64, t2118: f64, t2145: f64, t306: f64, t5952: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5989 = t5626 + t5677 + t5924 + t5987;
    let t5990 = t5989 * t158;
    let t5999 = t789 * t789;
    let t6000 = 1.0_f64 / t5999;
    let t6001 = t2119 * t799;
    let t6002 = t6000 * t6001;
    let t6005 = t2118 * t799;
    let t6006 = t6005 * t2145;
    let t6009 = t5952 * t306;
    (t5989, t5990, t5999, t6000, t6001, t6002, t6006, t6009)
}
