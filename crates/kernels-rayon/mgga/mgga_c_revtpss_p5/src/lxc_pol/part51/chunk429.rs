//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 429/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk429(t251: f64, t785: f64, t780: f64, t2439: f64, t212: f64, t860: f64, t689: f64, t779: f64, t887: f64, t211: f64, t784: f64, t209: f64) -> (f64, f64, f64, f64, f64) {
    let t2440 = t785 * t251;
    let t2441 = t2440 * t780;
    let t2443 = 0.65049603595885220126e-3_f64 * t2439 * t2441;
    let t2444 = t212 * t860;
    let t2445 = t2444 * t780;
    let t2446 = t689 * t2445;
    let t2448 = t779 * t887;
    let t2449 = t689 * t2448;
    let t2452 = 1.0_f64 / t784 / t211;
    let t2453 = t209 * t2452;
    (t2443, t2446, t2449, t2452, t2453)
}
