//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 909/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk909(t11594: f64, t11619: f64, t1045: f64, t373: f64, t1042: f64, t1034: f64, t360: f64, t11244: f64, t11240: f64, t3154: f64, t357: f64, t11249: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11620 = t11594 + t11619;
    let t11622 = t373 * t11620 * t1045;
    let t11623 = t1042 * t11622;
    let t11626 = t1034 * t1034;
    let t11627 = 1.0_f64 / t11626;
    let t11628 = t11627 * t360;
    let t11629 = t11628 * t11244;
    let t11630 = t11240 * t11629;
    let t11631 = t3154 * t357;
    let t11632 = t11249 * t11631;
    (t11620, t11623, t11627, t11630, t11631, t11632)
}
