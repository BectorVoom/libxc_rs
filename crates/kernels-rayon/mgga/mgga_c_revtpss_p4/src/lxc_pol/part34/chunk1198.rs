//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1198/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1198(t1032: f64, t11007: f64, t233: f64, t25372: f64, t1957: f64, t2718: f64, t25386: f64, t7015: f64, t9292: f64, t1955: f64, t7056: f64, t10867: f64, t867: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t93279 = t1032 * t11007;
    let t93280 = t93279 * t233;
    let t93281 = t25372 * t93280;
    let t93301 = t1957 * t2718;
    let t93302 = t25386 * t93301;
    let t93314 = t25372 * t93301;
    let t93317 = t25386 * t93280;
    let t93334 = 0.17073386770573548589e-1_f64 * t9292 * t7015;
    let t93349 = t1955 * t7056 * t11007;
    let t93355 = t867 * t10867;
    (t93281, t93302, t93314, t93317, t93334, t93349, t93355)
}
