//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1125/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1125(t27842: f64, t4074: f64, t4077: f64, t18091: f64, t27847: f64, t18089: f64, t18096: f64, t27846: f64, t4066: f64, t92: f64, t4082: f64, t4085: f64) -> (f64, f64, f64, f64) {
    let t29901 = t27842 * t4074 * t4077;
    let t29903 = t27847 * t18091;
    let t29908 = t18096 * t4066 * t27846 * t18089 * t92;
    let t29911 = t4082 * t27842 * t4085;
    (t29901, t29903, t29908, t29911)
}
