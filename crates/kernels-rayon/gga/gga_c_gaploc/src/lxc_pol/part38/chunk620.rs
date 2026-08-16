//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 620/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk620(t9097: f64, t9100: f64, t9108: f64, t9111: f64, t9113: f64, t9115: f64, t10209: f64, t3526: f64, t471: f64, t64: f64) -> (f64, f64) {
    let t11210 = -21.0_f64 / 128.0_f64 * t9097 + 147.0_f64 / 4096.0_f64 * t9100 - 63.0_f64 / 262144.0_f64 * t9108 + 21.0_f64 / 262144.0_f64 * t9111 - 49.0_f64 / 4096.0_f64 * t9113 + 7.0_f64 / 128.0_f64 * t9115;
    let t11218 = t11210 * t471 - 4.0_f64 / 3.0_f64 * t3526 * t64 + t10209 - 7.0_f64 / 128.0_f64 * t9097 + 21.0_f64 / 4096.0_f64 * t9100 - 7.0_f64 / 4096.0_f64 * t9113 + 7.0_f64 / 384.0_f64 * t9115;
    (t11210, t11218)
}
