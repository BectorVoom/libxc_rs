//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 495/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk495(t9097: f64, t9100: f64, t9108: f64, t9111: f64, t9113: f64, t9115: f64, t2287: f64, t871: f64, t3109: f64, t471: f64, t64: f64) -> (f64, f64, f64) {
    let t9117 = -21.0_f64 / 512.0_f64 * t9097 + 147.0_f64 / 16384.0_f64 * t9100 - 63.0_f64 / 1048576.0_f64 * t9108 + 21.0_f64 / 1048576.0_f64 * t9111 - 49.0_f64 / 16384.0_f64 * t9113 + 7.0_f64 / 512.0_f64 * t9115;
    let t9121 = t2287 * t871;
    let t9127 = t9117 * t471 - 4.0_f64 / 3.0_f64 * t3109 * t64 + t9121 / 2.0_f64 - 7.0_f64 / 512.0_f64 * t9097 + 21.0_f64 / 16384.0_f64 * t9100 - 7.0_f64 / 16384.0_f64 * t9113 + 7.0_f64 / 1536.0_f64 * t9115;
    (t9117, t9121, t9127)
}
