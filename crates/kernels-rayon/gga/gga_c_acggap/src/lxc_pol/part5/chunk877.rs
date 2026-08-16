//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 877/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk877(t3372: f64, t3665: f64, t1162: f64, t12313: f64, t1037: f64, t1165: f64, t945: f64, t1160: f64, t3430: f64, t3198: f64, t1111: f64, t301: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12770 = t3372 * t3665;
    let t12801 = t12313 * t1162;
    let t12804 = t12801 * t1165 * t1037 * t945;
    let t12813 = t1160 * t3430;
    let t12814 = t12813 * t3198;
    let t12816 = t1111 * t301;
    (t12770, t12801, t12804, t12813, t12814, t12816)
}
