//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1781/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1781(t1196: f64, t45187: f64, t45190: f64, t90357: f64, t90602: f64, t90629: f64, t90631: f64, t90634: f64, t90636: f64, t90640: f64, t90644: f64, t90855: f64, t90857: f64, t90860: f64, t90863: f64) -> (f64, f64) {
    let t90867 = 0.91082604192152556044e5_f64 * t1196 * t45187 * t90357 * t45190;
    let t90868 = -t90602 - t90629 - t90631 + t90634 - t90636 + t90640 + t90644 + t90855 + t90857 - t90860 - t90863 - t90867;
    (t90867, t90868)
}
