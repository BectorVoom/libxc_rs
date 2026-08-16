//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1158/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1158(t17060: f64, t2367: f64, t930: f64, t2668: f64, t42136: f64, t4947: f64, t17190: f64, t2586: f64, t940: f64, t17034: f64, t3917: f64, t42111: f64) -> (f64, f64, f64, f64) {
    let t51824 = t930 * t2367 * t17060;
    let t51827 = t2668 * t42136 * t4947;
    let t51903 = t940 * t2586 * t17190;
    let t51916 = t3917 * t42111 * t17034;
    (t51824, t51827, t51903, t51916)
}
