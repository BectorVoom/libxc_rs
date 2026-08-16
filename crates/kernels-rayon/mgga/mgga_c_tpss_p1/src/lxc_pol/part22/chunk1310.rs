//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1310/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1310(t10596: f64, t17964: f64, t10795: f64, t10799: f64, t3678: f64, t61033: f64, t10805: f64, t10581: f64, t3638: f64, t17954: f64, t339: f64, t3632: f64, t790: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t63901 = t17964 * t10596;
    let t63903 = t17964 * t10795;
    let t63905 = t17964 * t10799;
    let t63907 = t61033 * t3678;
    let t63909 = t17964 * t10805;
    let t63911 = t17964 * t10581;
    let t63913 = t61033 * t3638;
    let t63917 = t339 * t17954 * t790 * t3632;
    (t63901, t63903, t63905, t63907, t63909, t63911, t63913, t63917)
}
