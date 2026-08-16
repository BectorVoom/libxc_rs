//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 330/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk330(t1143: f64, t301: f64, t336: f64, t167: f64, t19: f64, t56: f64, t124: f64) -> (f64, f64, f64) {
    let t1145 = t336 * t1143 * t301;
    let t1149 = t56 * t167 * t19;
    let t1150 = t124 * t1149;
    (t1145, t1149, t1150)
}
