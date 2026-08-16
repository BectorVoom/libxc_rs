//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 615/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk615(t301: f64, t467: f64, t1680: f64, t694: f64, t3952: f64, t560: f64, t1679: f64, t811: f64, t4: f64, t483: f64) -> (f64, f64, f64, f64, f64) {
    let t3984 = t467 * t301;
    let t3986 = t694 * t1680 * t3984;
    let t3988 = t560 * t3952;
    let t3990 = t1679 * t3988 * t811;
    let t3992 = t483 * t4;
    (t3984, t3986, t3988, t3990, t3992)
}
