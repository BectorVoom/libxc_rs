//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 498/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk498(t1043: f64, t154: f64, t632: f64, t2289: f64, t888: f64, t892: f64, t287: f64, t891: f64, t275: f64, t273: f64, t276: f64, t2764: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2768 = t154 * t1043;
    let t2769 = t632 * t632;
    let t2770 = 1.0_f64 / t2769;
    let t2775 = 1.0_f64 / t2289;
    let t2787 = t888 * t892;
    let t2790 = t891 * t287;
    let t2791 = 1.0_f64 / t2790;
    let t2792 = t275 * t2791;
    let t2798 = 1.0_f64 / t276 / t273;
    let t2802 = 4.0_f64 / 9.0_f64 * t2764;
    (t2768, t2770, t2775, t2787, t2792, t2798, t2802)
}
