//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1266/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1266(t22: f64, t8950: f64, t8428: f64, t1122: f64, t3119: f64, t6548: f64, t1102: f64, t3058: f64, t4219: f64, t8738: f64, t8562: f64, t8577: f64) -> (f64, f64, f64, f64) {
    let t26140 = t22 * t8950;
    let t26141 = t26140 * t8428;
    let t26143 = t6548 * t1122 * t3119;
    let t26150 = 0.69263023597503453196e2_f64 * t1102 * t3058 * t8738 * t4219;
    let t26152 = 24.0_f64 * t8577 * t8562;
    (t26141, t26143, t26150, t26152)
}
