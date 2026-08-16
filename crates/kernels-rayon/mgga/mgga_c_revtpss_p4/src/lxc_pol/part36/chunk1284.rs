//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1284/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1284(t6864: f64, t94455: f64, t26024: f64, t6846: f64, t22061: f64, t25986: f64, t2661: f64, t22026: f64, t94550: f64, t22056: f64, t25972: f64, t22021: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t108590 = t94455 * t6864;
    let t108592 = t26024 * t6846;
    let t108601 = t2661 * t25986 * t22061;
    let t108604 = t2661 * t94550 * t22026;
    let t108608 = t25972 * t22056;
    let t108623 = t2661 * t25986 * t22021;
    (t108590, t108592, t108601, t108604, t108608, t108623)
}
