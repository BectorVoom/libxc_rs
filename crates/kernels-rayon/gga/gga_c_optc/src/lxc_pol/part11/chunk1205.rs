//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1205/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1205(t1150: f64, t17993: f64, t2367: f64, t12799: f64, t16007: f64, t1129: f64, t18092: f64, t3107: f64, t54753: f64, t3119: f64, t5096: f64, t1214: f64, t17615: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t55749 = t1150 * t2367 * t17993;
    let t55751 = t12799 * t16007;
    let t55753 = t18092 * t1129;
    let t55761 = t54753 * t3107;
    let t55768 = t3119 * t5096;
    let t55795 = t17615 * t1214;
    (t55749, t55751, t55753, t55761, t55768, t55795)
}
