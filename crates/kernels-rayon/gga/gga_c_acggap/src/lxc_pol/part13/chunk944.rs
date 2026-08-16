//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 944/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk944(t1967: f64, t7693: f64, t2092: f64, t7780: f64, t154: f64, t2096: f64, t31035: f64, t1156: f64, t2020: f64, t2016: f64, t7393: f64, t3036: f64, t597: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31503 = t1967 * t7693;
    let t31505 = t7780 * t2092;
    let t31508 = t31035 * t154 * t2096;
    let t31509 = 0.52805208333333333333e0_f64 * t31508;
    let t31510 = t2020 * t1156;
    let t31514 = t2016 * t7393;
    let t31520 = t3036 * t597;
    (t31503, t31505, t31509, t31510, t31514, t31520)
}
