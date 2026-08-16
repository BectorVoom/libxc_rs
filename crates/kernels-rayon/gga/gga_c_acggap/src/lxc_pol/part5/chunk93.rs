//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 93/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk93(t195: f64, t31: f64, t4: f64, t27: f64, t13: f64, t1: f64, t137: f64, t3: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t197 = t4 * t195 * t31;
    let t198 = 0.11073470983333333333e-2_f64 * t197;
    let t199 = t27 * t27;
    let t200 = 1.0_f64 / t199;
    let t201 = t13 * t200;
    let t202 = t137 * t1;
    let t203 = t3 * t6;
    (t198, t199, t200, t201, t202, t203)
}
