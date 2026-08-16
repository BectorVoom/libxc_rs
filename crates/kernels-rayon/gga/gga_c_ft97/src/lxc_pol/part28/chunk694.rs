//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 694/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk694(t3424: f64, t5916: f64, t23667: f64, t5899: f64, t23892: f64, t3188: f64, t23671: f64, t1642: f64, t586: f64, t23909: f64, t379: f64, t6656: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27064 = t5916 * t3424;
    let t27065 = t23667 * t27064;
    let t27066 = t5899 * t27065;
    let t27068 = t23892 * t3188;
    let t27069 = t23671 * t27068;
    let t27070 = t5899 * t27069;
    let t27072 = t1642 * t586;
    let t27073 = t23909 * t3188;
    let t27074 = t27072 * t27073;
    let t27075 = t5899 * t27074;
    let t27078 = t23671 * t6656 * t379;
    (t27064, t27066, t27068, t27070, t27072, t27073, t27075, t27078)
}
