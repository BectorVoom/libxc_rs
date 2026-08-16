//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 874/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk874(t14362: f64, t2630: f64, t1469: f64, t2609: f64, t706: f64, t1568: f64, t785: f64, t780: f64, t2439: f64, t2470: f64, t4480: f64, t2465: f64) -> (f64, f64, f64, f64, f64) {
    let t14363 = t14362 * t2630;
    let t14440 = t2609 * t1469;
    let t14441 = t706 * t14440;
    let t14472 = t785 * t1568;
    let t14473 = t14472 * t780;
    let t14474 = t2439 * t14473;
    let t14485 = t4480 * t2470;
    let t14486 = t2465 * t14485;
    (t14363, t14441, t14474, t14485, t14486)
}
