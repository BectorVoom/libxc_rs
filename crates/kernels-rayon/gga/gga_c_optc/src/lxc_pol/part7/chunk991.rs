//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 991/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk991(t1784: f64, t1793: f64, t6433: f64, t1757: f64, t534: f64, t539: f64, t6340: f64, t20814: f64, t36: f64, t88: f64, t1785: f64, t209: f64, t6485: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21874 = 0.57894567559743977359e3_f64 * t6433 * t1793 * t1784;
    let t21875 = t1784 * t1784;
    let t21878 = 6.0_f64 * t1757 * t21875 * t534;
    let t21879 = t539 * t6340;
    let t21880 = 48.0_f64 * t21879;
    let t21881 = 1.0_f64 / t20814;
    let t21884 = 840.0_f64 * t36 * t21881 * t88;
    let t21887 = 0.14246666666666666667e0_f64 * t209 * t6485 * t1785;
    (t21874, t21875, t21878, t21880, t21884, t21887)
}
