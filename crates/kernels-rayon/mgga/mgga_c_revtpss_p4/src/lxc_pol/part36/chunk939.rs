//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 939/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk939(t22267: f64, t4018: f64, t6864: f64, t9918: f64, t3930: f64, t6876: f64, t6862: f64, t72: f64, t686: f64, t10023: f64, t1385: f64, t6888: f64) -> (f64, f64, f64, f64, f64) {
    let t22268 = t4018 * t22267;
    let t22285 = t9918 * t6864;
    let t22292 = t3930 * t6876;
    let t22314 = t6862 * t72;
    let t22315 = t22314 * t686;
    let t22316 = t10023 * t22315;
    let t22321 = t1385 * t6888;
    (t22268, t22285, t22292, t22316, t22321)
}
