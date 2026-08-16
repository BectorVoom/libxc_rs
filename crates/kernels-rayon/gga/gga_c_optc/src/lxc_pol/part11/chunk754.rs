//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 754/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk754(t362: f64, t7298: f64, t1431: f64, t2352: f64, t1422: f64, t2300: f64, t322: f64, t7253: f64, t7256: f64, t24: f64, t2548: f64, t1382: f64, t7433: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10615 = t362 * t7298;
    let t10645 = t1431 * t2352;
    let t10760 = t1422 * t2300;
    let t10825 = t322 * t7253;
    let t10826 = t362 * t7256;
    let t10838 = t24 * t2548;
    let t10856 = t7433 * t1382;
    (t10615, t10645, t10760, t10825, t10826, t10838, t10856)
}
