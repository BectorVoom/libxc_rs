//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1185/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1185(t1846: f64, t7685: f64, t1817: f64, t31811: f64, t2030: f64, t301: f64, t8927: f64, t9552: f64, t2060: f64, t36222: f64, t372: f64, t1524: f64, t2288: f64) -> (f64, f64, f64, f64, f64) {
    let t40330 = t7685 * t1846;
    let t40332 = t31811 * t1817;
    let t40336 = t2030 * t8927 * t9552 * t301;
    let t40340 = t2060 * t36222 * t9552 * t372;
    let t40344 = t2060 * t8927 * t2288 * t1524;
    (t40330, t40332, t40336, t40340, t40344)
}
