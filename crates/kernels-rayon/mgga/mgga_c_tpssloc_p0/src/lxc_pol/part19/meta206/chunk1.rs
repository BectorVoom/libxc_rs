//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 880/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk880(t3041: f64, t884: f64, t3071: f64, t1023: f64, t2776: f64, t3051: f64, t820: f64) -> (f64, f64, f64, f64, f64) {
    let t10414 = t3041 * t884;
    let t10415 = t3071 * t10414;
    let t10418 = t2776 * t1023;
    let t10419 = t3071 * t10418;
    let t10422 = t820 * t3051;
    (t10414, t10415, t10418, t10419, t10422)
}
