//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2799/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2799(t21969: f64, t566: f64, t1450: f64, t22461: f64, t116: f64, t21813: f64, t21830: f64, t625: f64, t2289: f64, t5916: f64, t21877: f64, t1507: f64, t2357: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t75379 = t566 * t21969;
    let t75389 = t22461 * t1450;
    let t75439 = t21813 * t116;
    let t75526 = t625 * t21830;
    let t75540 = t2289 * t5916;
    let t75542 = t625 * t21877;
    let t75625 = t1507 * t2357;
    (t75379, t75389, t75439, t75526, t75540, t75542, t75625)
}
