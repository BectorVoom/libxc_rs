//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 850/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk850(t3441: f64, t4595: f64, t3440: f64, t141: f64, t16221: f64, t6917: f64, t1260: f64, t629: f64, t16287: f64, t5: f64, t659: f64, t13202: f64, t13260: f64, t13262: f64, t13277: f64, t13279: f64, t135: f64, t2011: f64, t3439: f64, t628: f64, t6925: f64, t6945: f64, t9651: f64, t9769: f64, t9782: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16411 = t3441 * t4595;
    let t16412 = t3440 * t16411;
    let t16416 = t6917 * t141 * t16221;
    let t16419 = t1260 * t4595;
    let t16420 = t629 * t16419;
    let t16428 = t5 * t16287;
    let t16429 = t629 * t16428;
    let t16432 = t5 * t16221;
    let t16433 = t629 * t16432;
    let t16438 = t659 * t141 * t16287;
    let t16442 = -0.30426065214260652491e0_f64 * t13202 + 0.16299677793353920977e0_f64 * t3439 * t16412 - 0.32599355586707841954e0_f64 * t135 * t16416 + 3.0_f64 / 16.0_f64 * t2011 * t16420 - 0.86207184773738515394e0_f64 * t9651 - 7.0_f64 / 16.0_f64 * t13260 + 7.0_f64 / 48.0_f64 * t13262 - 0.76065163035651631229e0_f64 * t13277 + 0.15213032607130326246e0_f64 * t13279 - t628 * t16429 / 48.0_f64 - t6945 * t16433 / 4.0_f64 - t6925 - 0.21551796193434628848e0_f64 * t9769 - 0.10866451862235947318e-1_f64 * t135 * t16438 - 35.0_f64 / 72.0_f64 * t9782;
    (t16411, t16412, t16416, t16419, t16420, t16429, t16433, t16438, t16442)
}
