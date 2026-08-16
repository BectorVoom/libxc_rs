//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3219/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3219(t61062: f64, t61085: f64, t150: f64, t190: f64, t2609: f64, t5825: f64, t706: f64, t18550: f64, t72: f64, t757: f64, t162: f64, t187: f64) -> (f64, f64, f64, f64) {
    let t61086 = t61062 + t61085;
    let t61088 = t150 * t61086 * t190;
    let t61090 = t706 * t2609 * t5825;
    let t61091 = 4.0_f64 * t61090;
    let t61093 = t18550 * t72 * t757;
    let t61094 = 0.36622894612013090108e-3_f64 * t61093;
    let t61097 = 0.19751673498613801407e-1_f64 * t61086 * t162 * t187;
    (t61088, t61091, t61094, t61097)
}
