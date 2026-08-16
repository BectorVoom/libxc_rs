//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2623/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2623(t18263: f64, t2615: f64, t2475: f64, t5962: f64, t10696: f64, t5966: f64, t18616: f64, t221: f64, t2484: f64, t2485: f64, t10815: f64, t5980: f64) -> (f64, f64, f64, f64, f64) {
    let t62302 = t18263 * t2615;
    let t62351 = t2475 * t5962;
    let t62361 = t10696 * t5966;
    let t62392 = t2484 * t2485 * t221 * t18616;
    let t62399 = t10815 * t5980;
    (t62302, t62351, t62361, t62392, t62399)
}
