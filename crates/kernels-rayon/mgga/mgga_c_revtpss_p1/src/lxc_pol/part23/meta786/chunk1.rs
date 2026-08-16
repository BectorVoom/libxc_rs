//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2598/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2598(t14369: f64, t4186: f64, t4401: f64, t2439: f64, t6041: f64, t780: f64, t785: f64, t18821: f64, t2471: f64, t18814: f64, t2435: f64, t14476: f64, t1580: f64, t689: f64) -> (f64, f64, f64, f64, f64) {
    let t61315 = t4401 * t14369 * t4186;
    let t61324 = t2439 * t785 * t6041 * t780;
    let t61330 = t18821 * t2471;
    let t61337 = t2435 * t18814;
    let t61344 = t689 * t14476 * t1580;
    (t61315, t61324, t61330, t61337, t61344)
}
