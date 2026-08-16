//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1352/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1352(t3126: f64, t8487: f64, t3132: f64, t4357: f64, t24502: f64, t465: f64, t8970: f64, t26911: f64, t3133: f64, t4386: f64, t8493: f64, t9189: f64) -> (f64, f64, f64, f64, f64) {
    let t26936 = t8487 * t3126;
    let t26938 = t3132 * t26936 * t4357;
    let t26940 = t465 * t24502;
    let t26941 = t26940 * t8970;
    let t26944 = t3132 * t26911 * t3133;
    let t26947 = t4386 * t9189 * t8493;
    (t26936, t26938, t26941, t26944, t26947)
}
