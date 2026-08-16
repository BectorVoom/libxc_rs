//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1116/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1116(t1358: f64, t7492: f64, t689: f64, t2098: f64, t786: f64, t1364: f64, t7250: f64, t7257: f64, t7260: f64, t7267: f64, t7253: f64, t7265: f64, t7272: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7493 = t7492 * t1358;
    let t7495 = 0.54878743191129263322e-2_f64 * t689 * t7493;
    let t7496 = t786 * t2098;
    let t7498 = 0.9757440539382783019e-2_f64 * t7496 * t1364;
    let t7499 = 7.0_f64 / 144.0_f64 * t7250;
    let t7501 = 0.28582678745379824648e-4_f64 * t7257;
    let t7502 = 0.50820002809285328225e-4_f64 * t7260;
    let t7504 = 0.40015750243531754507e-2_f64 * t7267;
    let t7506 = -t7499 - t7253 / 24.0_f64 - t7501 + t7502 - 0.85748036236139473944e-3_f64 * t7265 - t7504 - 0.34299214494455789578e-2_f64 * t7272;
    (t7493, t7495, t7496, t7498, t7499, t7501, t7502, t7504, t7506)
}
