//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1095/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1095(t10009: f64, t2013: f64, t10004: f64, t5676: f64, t1645: f64, t7124: f64, t23309: f64, t7372: f64, t1966: f64, t9801: f64, t5640: f64, t9807: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28378 = t2013 * t10009;
    let t28381 = 0.11916829983950142223e0_f64 * t5676 * t10004;
    let t28387 = t1645 * t7124;
    let t28406 = 0.59584149919750711116e-1_f64 * t23309 * t7372;
    let t28407 = t1966 * t9801;
    let t28408 = 0.51123901271894332901e1_f64 * t28407;
    let t28409 = t5640 * t9807;
    (t28378, t28381, t28387, t28406, t28408, t28409)
}
