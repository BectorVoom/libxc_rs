//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1712/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1712(t11659: f64, t4910: f64, t3117: f64, t1016: f64, t697: f64, t1011: f64, t1010: f64, t2270: f64) -> (f64, f64, f64, f64, f64) {
    let t11876 = t11659 * t4910;
    let t11877 = t3117 * t11876;
    let t11880 = t697 * t1016;
    let t11881 = t1011 * t11880;
    let t11883 = t2270 * t1010;
    (t11876, t11877, t11880, t11881, t11883)
}
