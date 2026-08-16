//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 650/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk650(t1312: f64, t2320: f64, t2322: f64, t2327: f64, t2371: f64, t670: f64, t93: f64, t1330: f64, t72: f64, t757: f64, t530: f64, t566: f64) -> (f64, f64, f64, f64) {
    let t3821 = 2.0_f64 * t1312 * t2371 + 4.0_f64 * t2322 * t670 + 2.0_f64 * t2327 * t93 + t2320;
    let t3825 = t1330 * t72;
    let t3826 = t3825 * t757;
    let t3827 = 0.36622894612013090108e-3_f64 * t3826;
    let t3828 = t530 * t566;
    (t3821, t3825, t3827, t3828)
}
