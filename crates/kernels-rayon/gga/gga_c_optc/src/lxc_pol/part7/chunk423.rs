//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 423/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk423(t2051: f64, t2052: f64, t138: f64, t637: f64, t120: f64, t658: f64, t124: f64, t1928: f64, t1948: f64, t121: f64, t641: f64, t642: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2053 = t2051 + t2052;
    let t2057 = t637 * t138;
    let t2060 = t120 * t658;
    let t2061 = t124 * t1928;
    let t2064 = t124 * t1948;
    let t2067 = -0.12897460341341234505e3_f64 * t2053 * t121 * t124 + 0.7738476204804740703e3_f64 * t2057 * t642 - 0.15476952409609481406e4_f64 * t2060 * t2061 + 0.38692381024023703515e3_f64 * t641 * t2064;
    (t2053, t2057, t2060, t2061, t2064, t2067)
}
