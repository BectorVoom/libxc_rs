//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 532/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk532(t2764: f64, t273: f64, t241: f64, t63: f64, t281: f64, t283: f64, t699: f64, t909: f64, t976: f64, t891: f64, t275: f64, t290: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2810 = 0.39862222222222222223e0_f64 * t2764;
    let t2815 = 1.0_f64/f64::sqrt(t273);
    let t2820 = t63 * t241;
    let t2822 = t281 * t2820 * t283;
    let t2823 = 0.13692777777777777778e0_f64 * t2822;
    let t2824 = t699 * t909;
    let t2826 = t241 * t976;
    let t2840 = t891 * t891;
    let t2841 = 1.0_f64 / t2840;
    let t2842 = t275 * t2841;
    let t2843 = t290 * t290;
    (t2810, t2815, t2820, t2822, t2823, t2824, t2826, t2842, t2843)
}
