//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1323/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1323(t11906: f64, t11909: f64, t11914: f64, t11921: f64, t1670: f64, t1674: f64, t20034: f64, t20036: f64, t20037: f64, t20038: f64, t20039: f64, t3984: f64, t4099: f64, t5645: f64, t6596: f64, t694: f64, t695: f64) -> f64 {
    let t24643 = 6.0_f64 * t1670 * t4099 * t694 + 24.0_f64 * t1674 * t5645 * t695 + 12.0_f64 * t3984 * t6596 * t694 + t11906 - t11909 - t11914 - t11921 + t20034 + t20036 + t20037 + t20038 - t20039;
    t24643
}
