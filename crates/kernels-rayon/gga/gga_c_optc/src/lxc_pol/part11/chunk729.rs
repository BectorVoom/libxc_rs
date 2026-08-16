//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 729/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk729(t8642: f64, t8639: f64, t1035: f64, t3016: f64, t375: f64, t3019: f64, t388: f64, t1084: f64, t3057: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8643 = 0.36514074074074074075e0_f64 * t8642;
    let t8662 = 28.0_f64 / 27.0_f64 * t8639;
    let t8685 = 1.0_f64 / t3016 / t1035;
    let t8686 = t375 * t8685;
    let t8688 = 1.0_f64 / t3019 / t388;
    let t8697 = 1.0_f64 / t3057 / t1084;
    (t8643, t8662, t8685, t8686, t8688, t8697)
}
