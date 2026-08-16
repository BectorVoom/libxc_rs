//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 858/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk858(t25899: f64, t25901: f64, t25894: f64, t25898: f64, t212: f64, t7274: f64, t1358: f64, t689: f64, t2022: f64, t785: f64, t2439: f64, t1032: f64, t1419: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25902 = t25899 * t25901;
    let t25904 = t25894 * t25898;
    let t25905 = t25904 * t25901;
    let t25912 = t212 * t7274;
    let t25913 = t25912 * t1358;
    let t25914 = t689 * t25913;
    let t25916 = t785 * t2022;
    let t25917 = t25916 * t1358;
    let t25919 = 0.65049603595885220126e-3_f64 * t2439 * t25917;
    let t25920 = t1419 * t1032;
    (t25902, t25904, t25905, t25914, t25919, t25920)
}
