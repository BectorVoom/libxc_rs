//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2622/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2622(t14370: f64, t18259: f64, t18562: f64, t2626: f64, t14330: f64, t5819: f64, t606: f64, t749: f64, t162: f64, t50089: f64, t2609: f64, t5944: f64) -> (f64, f64, f64, f64, f64) {
    let t62274 = t18259 * t14370;
    let t62276 = t18562 * t2626;
    let t62282 = t14330 * t749 * t5819 * t606;
    let t62291 = t50089 * t162;
    let t62300 = t5944 * t2609;
    (t62274, t62276, t62282, t62291, t62300)
}
