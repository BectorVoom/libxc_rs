//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1321/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1321(t1662: f64, t467: f64, t11874: f64, t1268: f64, t1674: f64, t1679: f64, t1713: f64, t1734: f64, t20016: f64, t20018: f64, t20019: f64, t20021: f64, t20022: f64, t20023: f64, t2637: f64, t3988: f64, t5651: f64, t6614: f64, t694: f64, t695: f64) -> f64 {
    let t24605 = t1662 * t467;
    let t24617 = -t1268 * t1679 * t6614 - 6.0_f64 * t1674 * t1713 * t2637 + 12.0_f64 * t1674 * t5651 * t695 + 8.0_f64 * t1679 * t24605 * t3988 - 3.0_f64 * t1734 * t2637 * t694 + t11874 - t20016 + t20018 - t20019 + t20021 - t20022 - t20023;
    t24617
}
