//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 810/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk810(t25913: f64, t689: f64, t2022: f64, t785: f64, t1358: f64, t2439: f64, t1032: f64, t1419: f64, t1955: f64, t545: f64, t9656: f64, t4075: f64, t7282: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25914 = t689 * t25913;
    let t25916 = t785 * t2022;
    let t25917 = t25916 * t1358;
    let t25919 = 0.65049603595885220126e-3_f64 * t2439 * t25917;
    let t25920 = t1419 * t1032;
    let t25921 = t1955 * t25920;
    let t25924 = t9656 * t545;
    let t25929 = t7282 * t4075;
    (t25914, t25919, t25920, t25921, t25924, t25929)
}
