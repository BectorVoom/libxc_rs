//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1150/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1150(t6966: f64, t6981: f64, t164: f64, t7084: f64, t5257: f64, t6972: f64, t6882: f64, t6891: f64, t6895: f64, t6899: f64, t6963: f64, t1721: f64, t19965: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19995 = t6966 * t6981;
    let t19997 = t7084 * t164;
    let t20002 = t5257 * t6972;
    let t20004 = t6966 * t6882;
    let t20010 = t6895 * t6891;
    let t20011 = t20010 * t6899;
    let t20017 = t5257 * t6963;
    let t20019 = t19965 * t1721;
    (t19995, t19997, t20002, t20004, t20011, t20017, t20019)
}
