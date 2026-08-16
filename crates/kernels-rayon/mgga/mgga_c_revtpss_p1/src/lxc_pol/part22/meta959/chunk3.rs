//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3220/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3220(t49897: f64, t4343: f64, t890: f64, t18871: f64, t1940: f64, t2403: f64, t2408: f64, t2832: f64, t39442: f64, t4556: f64, t61031: f64, t61032: f64, t61033: f64, t61039: f64, t61088: f64, t61091: f64, t61094: f64, t61097: f64) -> (f64, f64) {
    let t61101 = 0.11696447245269292414e1_f64 * t49897;
    let t61102 = t4343 * t890;
    let t61106 = 2.0_f64 * t18871 * t1940 * t2832 + 2.0_f64 * t1940 * t2408 * t61033 - 12.0_f64 * t2403 * t4556 * t61102 + t39442 + t61031 + t61032 + t61039 + t61088 + t61091 - t61094 + t61097 - t61101;
    (t61101, t61106)
}
