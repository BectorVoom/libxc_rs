//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1458/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1458(t11509: f64, t6205: f64, t2967: f64, t6152: f64, t3011: f64, t6184: f64, t2942: f64, t2923: f64, t6104: f64, t3056: f64, t6234: f64, t378: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t64043 = t6205 * t11509;
    let t64060 = t6152 * t2967;
    let t64125 = t6184 * t3011;
    let t64319 = t6152 * t2942;
    let t64336 = t6104 * t2923;
    let t64686 = t6234 * t3056;
    let t64687 = t64686 * t378;
    (t64043, t64060, t64125, t64319, t64336, t64686, t64687)
}
