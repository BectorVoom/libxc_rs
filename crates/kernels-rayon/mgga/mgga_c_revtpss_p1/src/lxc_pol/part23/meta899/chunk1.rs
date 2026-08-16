//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2860/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2860(t18305: f64, t4186: f64, t4401: f64, t18576: f64, t62291: f64, t62302: f64, t50892: f64, t50893: f64, t189: f64, t22671: f64, t606: f64, t177: f64, t23211: f64, t762: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t77036 = 36.0_f64 * t4401 * t18305 * t4186;
    let t77038 = 72.0_f64 * t62291 * t18576;
    let t77039 = 12.0_f64 * t62302;
    let t77040 = 3.0_f64 * t50892;
    let t77041 = 0.31168546390226634765e3_f64 * t50893;
    let t77042 = t189 * t22671;
    let t77045 = 12.0_f64 * t4401 * t77042 * t606;
    let t77047 = t23211 * t177 * t762;
    (t77036, t77038, t77039, t77040, t77041, t77045, t77047)
}
