//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 860/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk860(t30019: f64, t883: f64, t2300: f64, t9074: f64, t12360: f64, t2312: f64, t2321: f64, t882: f64, t9493: f64, t2325: f64, t29661: f64, t2326: f64, t9079: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39776 = t883 * t30019;
    let t39778 = t9074 * t2300 * t39776;
    let t39791 = t2312 * t12360;
    let t39794 = t882 * t9493 * t2321;
    let t39798 = t882 * t2325 * t883 * t29661;
    let t39805 = t9074 * t9079 * t2326;
    (t39776, t39778, t39791, t39794, t39798, t39805)
}
