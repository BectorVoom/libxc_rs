//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2319/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2319(t15689: f64, t7310: f64, t27674: f64, t3548: f64, t15753: f64, t27608: f64, t7321: f64, t1222: f64, t27586: f64, t15357: f64, t15560: f64, t2134: f64, t24650: f64, t27580: f64, t27692: f64, t27714: f64, t460: f64, t7320: f64, t8040: f64, t86282: f64, t86296: f64, t86324: f64) -> f64 {
    let t95507 = t7310 * t15689 / 432.0_f64;
    let t95511 = t27674 * t3548 / 162.0_f64;
    let t95512 = t7310 * t15753;
    let t95515 = 0.20186378047070195428e-3_f64 * t27608 * t7321;
    let t95517 = t27586 * t1222 / 1152.0_f64;
    let t95518 = -0.20186378047070195428e-3_f64 * t24650 * t27692 - 0.10093189023535097714e-3_f64 * t86296 * t8040 + 0.20186378047070195428e-3_f64 * t27714 * t7321 - 0.10093189023535097714e-3_f64 * t2134 * t15357 * t460 * t7320 - t86324 * t15560 / 1152.0_f64 - 0.10093189023535097714e-3_f64 * t86282 - t95507 - 0.16149102437656156342e-2_f64 * t27580 * t7321 + t95511 + t95512 / 1296.0_f64 - t95515 + t95517;
    t95518
}
