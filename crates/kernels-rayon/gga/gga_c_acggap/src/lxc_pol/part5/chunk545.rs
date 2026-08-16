//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 545/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk545(t3125: f64, t3126: f64, t3124: f64, t1049: f64, t1056: f64, t137: f64, t167: f64, t130: f64, t985: f64, t138: f64, t1046: f64, t134: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3127 = t3125 * t3126;
    let t3128 = t3124 * t3127;
    let t3130 = t1049 * t1056;
    let t3132 = t167 * t137;
    let t3140 = t130 * t985;
    let t3141 = t3140 * t138;
    let t3142 = 70.0_f64 / 27.0_f64 * t3141;
    let t3143 = t1046 * t134;
    (t3127, t3128, t3130, t3132, t3140, t3141, t3142, t3143)
}
