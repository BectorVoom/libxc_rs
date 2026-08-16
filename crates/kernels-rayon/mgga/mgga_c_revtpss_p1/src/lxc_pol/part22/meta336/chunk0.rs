//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1794/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1794(t11044: f64, t2467: f64, t2828: f64, t676: f64, t123: f64, t2465: f64, t2410: f64, t261: f64, t2832: f64, t892: f64, t2408: f64, t2411: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11045 = t11044 * t2467;
    let t11049 = t676 * t2828;
    let t11050 = t123 * t11049;
    let t11051 = t2465 * t11050;
    let t11064 = 1.0_f64 / t2410 / t261;
    let t11075 = t2832 * t892;
    let t11084 = t2408 * t2411;
    (t11045, t11049, t11050, t11051, t11064, t11075, t11084)
}
