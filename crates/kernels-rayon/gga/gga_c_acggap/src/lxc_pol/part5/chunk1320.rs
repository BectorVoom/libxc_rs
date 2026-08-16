//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1320/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1320(t1662: f64, t301: f64, t11856: f64, t1298: f64, t14866: f64, t1674: f64, t1680: f64, t1946: f64, t20007: f64, t20008: f64, t20009: f64, t20010: f64, t20011: f64, t20013: f64, t5403: f64, t6592: f64, t694: f64, t922: f64) -> f64 {
    let t24589 = t301 * t1662;
    let t24601 = 12.0_f64 * t1298 * t5403 * t694 + 6.0_f64 * t1674 * t6592 * t922 - 12.0_f64 * t1680 * t24589 * t694 + 12.0_f64 * t14866 * t1946 + t11856 + t20007 - t20008 - t20009 - t20010 + t20011 + t20013;
    t24601
}
