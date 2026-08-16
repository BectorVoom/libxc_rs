//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 945/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk945(t242: f64, t2841: f64, t9523: f64, t1125: f64, t3060: f64, t3081: f64, t3080: f64, t215: f64, t442: f64, t68: f64, t441: f64, t3074: f64) -> (f64, f64, f64, f64) {
    let t9525 = t242 * t9523 * t2841;
    let t9526 = t1125 * t9525;
    let t9529 = t242 * t3060 * t3081;
    let t9530 = t3080 * t9529;
    let t9533 = t215 * t68 * t442;
    let t9535 = 5.0_f64 / 1296.0_f64 * t441 * t9533;
    let t9537 = t242 * t3060 * t3074;
    (t9526, t9530, t9535, t9537)
}
