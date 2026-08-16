//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 518/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk518(t313: f64, t934: f64, t1045: f64, t3293: f64, t1109: f64, t2952: f64, t345: f64, t1035: f64, t346: f64, t3074: f64, t1114: f64, t3096: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3294 = t313 * t934;
    let t3295 = t3294 * t1045;
    let t3296 = t3293 * t3295;
    let t3299 = t1109 * t2952;
    let t3300 = t345 * t3299;
    let t3303 = t346 * t1035;
    let t3304 = t3303 * t3074;
    let t3305 = t345 * t3304;
    let t3308 = t1114 * t3096;
    (t3295, t3296, t3299, t3300, t3303, t3304, t3305, t3308)
}
