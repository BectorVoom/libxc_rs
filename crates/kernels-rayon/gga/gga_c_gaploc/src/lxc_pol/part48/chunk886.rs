//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 886/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk886(t1016: f64, t10624: f64, t1382: f64, t11969: f64, t2592: f64, t2798: f64, t2801: f64, t33959: f64, t32100: f64, t10301: f64, t8045: f64, t11714: f64, t7324: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45123 = 4.0_f64 * t1382 * t1016 * t10624;
    let t45124 = t2592 * t11969;
    let t45126 = 2.0_f64 * t2798 * t10624;
    let t45130 = 4.0_f64 * t33959 * t2801;
    let t45132 = 2.0_f64 * t32100 * t1016;
    let t45134 = 4.0_f64 * t8045 * t10301;
    let t45141 = 4.0_f64 * t7324 * t11714;
    (t45123, t45124, t45126, t45130, t45132, t45134, t45141)
}
