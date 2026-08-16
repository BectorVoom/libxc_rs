//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1046/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1046(t33778: f64, t955: f64, t13064: f64, t2684: f64, t7354: f64, t10867: f64, t1423: f64, t3247: f64, t41330: f64, t41337: f64, t41340: f64, t13077: f64, t28439: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43901 = t955 * t33778;
    let t43904 = t2684 * t7354 * t13064;
    let t43907 = t10867 * t1423 * t3247;
    let t43908 = 0.17875244975925213335e0_f64 * t43907;
    let t43909 = 0.11502877786176224903e1_f64 * t41330;
    let t43910 = 0.11916829983950142223e0_f64 * t41337;
    let t43911 = 0.89376224879626066674e-1_f64 * t41340;
    let t43912 = t13077 * t28439;
    (t43901, t43904, t43908, t43909, t43910, t43911, t43912)
}
