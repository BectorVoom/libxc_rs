//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1152/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1152(t31158: f64, t9561: f64, t1397: f64, t6851: f64, t9562: f64, t21005: f64, t544: f64, t6466: f64, t900: f64, t549: f64, t6520: f64, t7025: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31160 = 0.3575048995185042667e0_f64 * t9561 * t31158;
    let t31163 = 0.17875244975925213335e0_f64 * t1397 * t6851 * t9562;
    let t31166 = 0.17875244975925213335e0_f64 * t544 * t21005 * t9562;
    let t31167 = t900 * t6466;
    let t31169 = 0.89376224879626066674e-1_f64 * t9561 * t31167;
    let t31172 = 0.11916829983950142223e0_f64 * t7025 * t549 * t6520;
    (t31160, t31163, t31166, t31167, t31169, t31172)
}
