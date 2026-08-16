//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1407/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1407(t1037: f64, t1056: f64, t59103: f64, t59116: f64, t59132: f64, t59147: f64, t17427: f64, t34422: f64, t58812: f64, t58820: f64, t58822: f64, t58834: f64, t58836: f64, t58864: f64, t58884: f64, t58888: f64, t59086: f64, t59088: f64) -> (f64, f64, f64) {
    let t59152 = 1.0_f64 * t1037 * (t59103 + t59116 + t59132 + t59147) * t1056;
    let t59154 = 0.20690005882282467367e4_f64 * t34422 * t17427;
    let t59155 = t58812 - t58820 - t58822 + t58834 + t58836 + t58864 + t58884 - t58888 - t59086 + t59088 + t59152 + t59154;
    (t59152, t59154, t59155)
}
