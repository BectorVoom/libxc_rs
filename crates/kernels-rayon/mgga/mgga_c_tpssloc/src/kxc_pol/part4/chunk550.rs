//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 550/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk550(t888: f64, t892: f64, t287: f64, t891: f64, t275: f64, t273: f64, t276: f64, t2764: f64, t241: f64, t63: f64, t281: f64, t283: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2787 = t888 * t892;
    let t2790 = t891 * t287;
    let t2791 = 1.0_f64 / t2790;
    let t2792 = t275 * t2791;
    let t2798 = 1.0_f64 / t276 / t273;
    let t2802 = 4.0_f64 / 9.0_f64 * t2764;
    let t2810 = 0.39862222222222222223e0_f64 * t2764;
    let t2815 = 1.0_f64/f64::sqrt(t273);
    let t2820 = t63 * t241;
    let t2822 = t281 * t2820 * t283;
    (t2787, t2791, t2792, t2798, t2802, t2810, t2815, t2820, t2822)
}
