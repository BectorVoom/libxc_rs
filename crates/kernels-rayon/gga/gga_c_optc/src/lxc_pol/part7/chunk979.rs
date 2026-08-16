//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 979/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk979(t322: f64, t7253: f64, t362: f64, t7256: f64, t24: f64, t2548: f64, t1013: f64, t2434: f64, t8384: f64, t19: f64, t2606: f64, t3813: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10825 = t322 * t7253;
    let t10826 = t362 * t7256;
    let t10838 = t24 * t2548;
    let t10845 = t2434 * t1013;
    let t10849 = t8384 * t1013;
    let t10887 = t19 * t2606;
    let t10888 = t10887 * t3813;
    (t10825, t10826, t10838, t10845, t10849, t10888)
}
