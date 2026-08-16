//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 962/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk962(t43425: f64, t10736: f64, t28412: f64, t913: f64, t2679: f64, t3451: f64, t9796: f64, t3038: f64, t6119: f64, t787: f64, t9755: f64, t11112: f64, t2617: f64, t7810: f64) -> (f64, f64, f64, f64, f64) {
    let t43426 = 0.42603251059911944084e0_f64 * t43425;
    let t43432 = t28412 * t913 * t10736;
    let t43433 = 0.59584149919750711116e-1_f64 * t43432;
    let t43435 = t9796 * t3451 * t2679;
    let t43440 = 0.27805936629216998521e0_f64 * t787 * t9755 * t3038 * t6119;
    let t43442 = t7810 * t11112 * t2617;
    (t43426, t43433, t43435, t43440, t43442)
}
