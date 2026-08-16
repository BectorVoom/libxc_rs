//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1427/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1427(t2516: f64, t5571: f64, t5566: f64, t72: f64, t757: f64, t1320: f64, t5567: f64, t5569: f64, t9395: f64, t9398: f64, t1353: f64, t1448: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13611 = t5571 * t2516;
    let t13612 = 0.5848223622634646207e0_f64 * t13611;
    let t13613 = t5566 * t72;
    let t13615 = 0.36622894612013090108e-3_f64 * t13613 * t757;
    let t13620 = 8.0_f64 * t1320 * t5567;
    let t13621 = t1320 * t5569;
    let t13622 = 8.0_f64 * t13621;
    let t13623 = 4.0_f64 * t9395;
    let t13624 = 16.0_f64 * t9398;
    let t13625 = t1353 * t1448;
    (t13612, t13615, t13620, t13622, t13623, t13624, t13625)
}
