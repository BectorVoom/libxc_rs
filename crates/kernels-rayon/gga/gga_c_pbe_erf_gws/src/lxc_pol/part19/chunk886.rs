//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 886/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk886(t2358: f64, t3916: f64, t3703: f64, t831: f64, t6148: f64, t830: f64, t1109: f64, t2395: f64, t829: f64, t3028: f64, t1145: f64, t858: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9815 = t3916 * t2358;
    let t9818 = t831 * t3703;
    let t9820 = t6148 * t830 * t9818;
    let t9827 = t829 * t830 * t2395 * t1109;
    let t9832 = t829 * t830 * t831 * t3028;
    let t9837 = t858 * t1145;
    (t9815, t9818, t9820, t9827, t9832, t9837)
}
