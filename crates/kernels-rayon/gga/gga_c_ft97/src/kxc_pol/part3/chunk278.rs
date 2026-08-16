//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 278/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk278(t1023: f64, t1054: f64, t1058: f64, t1060: f64, t149: f64, t165: f64, t184: f64, t632: f64, t72: f64, t920: f64, t1002: f64, t641: f64, t927: f64) -> (f64, f64, f64, f64) {
    let t1063 = -t1023 * t165 - t1058 * t149 - 2.0_f64 * t1054 + 2.0_f64 * t1060;
    let t1064 = t1063 * t184;
    let t1068 = t72 * t632 * t920;
    let t1073 = 0.234754e0_f64 * t1002 - t641 - 0.14443083333333333333e0_f64 * t927;
    (t1063, t1064, t1068, t1073)
}
