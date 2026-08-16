//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1027/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1027(t146: f64, t147: f64, t6567: f64, t155: f64, t158: f64, t6165: f64, t2004: f64, t2123: f64, t115: f64, t658: f64, t5: f64, t2219: f64) -> (f64, f64, f64, f64, f64) {
    let t23163 = t146 * t147 * t6567;
    let t23171 = t155 * t158 * t6165;
    let t23219 = t2123 * t2004;
    let t23269 = t658 * t115;
    let t23270 = t23269 * t5;
    let t23315 = t2219 * t2219;
    (t23163, t23171, t23219, t23270, t23315)
}
