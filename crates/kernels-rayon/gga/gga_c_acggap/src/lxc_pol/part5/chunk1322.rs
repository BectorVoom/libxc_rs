//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1322/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1322(t495: f64, t811: f64, t11898: f64, t11900: f64, t1272: f64, t1670: f64, t1674: f64, t1679: f64, t1680: f64, t20025: f64, t20027: f64, t20031: f64, t20032: f64, t20033: f64, t20092: f64, t3988: f64, t4818: f64, t5392: f64, t694: f64, t96: f64) -> f64 {
    let t24623 = t495 * t811;
    let t24633 = 12.0_f64 * t1272 * t20092 * t96 + 24.0_f64 * t1670 * t1674 * t4818 - 2.0_f64 * t1679 * t1680 * t5392 + 12.0_f64 * t24623 * t3988 * t694 + t11898 + t11900 + t20025 + t20027 - t20031 + t20032 + t20033;
    t24633
}
