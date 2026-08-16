//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1465/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1465(t4021: f64, t9976: f64, t1384: f64, t235: f64) -> (f64, f64, f64, f64) {
    let t9977 = t9976 * t4021;
    let t9989 = t1384 * t1384;
    let t9990 = 1.0_f64 / t9989;
    let t9991 = t9990 * t235;
    (t9977, t9989, t9990, t9991)
}
