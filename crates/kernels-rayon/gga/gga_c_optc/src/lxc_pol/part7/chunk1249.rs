//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1249/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1249(t8229: f64, t921: f64, t7895: f64, t947: f64, t7373: f64, t7433: f64, t8127: f64, t8129: f64, t2367: f64, t7920: f64, t930: f64, t2670: f64, t288: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25791 = t921 * t8229;
    let t25793 = t947 * t7895;
    let t25797 = t7433 * t7373;
    let t25799 = t8127 * t25797 * t8129;
    let t25804 = t930 * t2367 * t7920;
    let t25806 = t288 * t2670;
    (t25791, t25793, t25797, t25799, t25804, t25806)
}
