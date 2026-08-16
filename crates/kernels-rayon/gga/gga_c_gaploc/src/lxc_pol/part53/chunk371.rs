//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 371/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk371(t157: f64, t3113: f64, t3095: f64, t3108: f64, t3109: f64, t3111: f64, t471: f64) -> (f64, f64) {
    let t3114 = t157 * t3113;
    let t3116 = t3109 * t471 + t3111 / 2.0_f64 + t3095 - t3108 - t3114 / 2.0_f64;
    (t3114, t3116)
}
