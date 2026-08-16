//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 813/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk813(t3431: f64, t5241: f64, t2679: f64, t9805: f64, t20671: f64, t28069: f64, t33148: f64, t10736: f64, t28412: f64, t913: f64, t3451: f64, t9796: f64) -> (f64, f64, f64, f64) {
    let t43419 = t5241 * t3431;
    let t43421 = t9805 * t43419 * t2679;
    let t43425 = t28069 * t20671 * t33148;
    let t43432 = t28412 * t913 * t10736;
    let t43435 = t9796 * t3451 * t2679;
    (t43421, t43425, t43432, t43435)
}
