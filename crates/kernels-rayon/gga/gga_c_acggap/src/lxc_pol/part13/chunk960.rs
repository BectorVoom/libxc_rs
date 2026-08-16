//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 960/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk960(t377: f64, t7732: f64, t947: f64, t31404: f64, t7507: f64, t7517: f64, t31491: f64, t7381: f64, t922: f64, t2020: f64, t7855: f64, t3088: f64, t7646: f64) -> (f64, f64, f64, f64, f64) {
    let t31863 = t377 * t7732;
    let t31864 = t31863 * t947;
    let t31867 = t7507 * t31404 * t7517;
    let t31868 = 0.1383716060742582691e-1_f64 * t31867;
    let t31870 = t31491 * t7381 * t922;
    let t31872 = t2020 * t7855;
    let t31878 = t3088 * t7646;
    (t31864, t31868, t31870, t31872, t31878)
}
