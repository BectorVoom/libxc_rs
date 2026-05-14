//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 800/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk800<F: Float>(t3441: F, t4595: F, t3440: F, t141: F, t16221: F, t6917: F, t1260: F, t629: F, t16287: F, t5: F, t659: F, t13202: F, t13260: F, t13262: F, t13277: F, t13279: F, t135: F, t2011: F, t3439: F, t628: F, t6925: F, t6945: F, t9651: F, t9769: F, t9782: F) -> (F, F, F, F, F, F, F, F, F) {
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
    let t16442 = -0.30426065214260652491e0 * t13202 + 0.16299677793353920977e0 * t3439 * t16412 - 0.32599355586707841954e0 * t135 * t16416 + 3.0 / 16.0 * t2011 * t16420 - 0.86207184773738515394e0 * t9651 - 7.0 / 16.0 * t13260 + 7.0 / 48.0 * t13262 - 0.76065163035651631229e0 * t13277 + 0.15213032607130326246e0 * t13279 - t628 * t16429 / 48.0 - t6945 * t16433 / 4.0 - t6925 - 0.21551796193434628848e0 * t9769 - 0.10866451862235947318e-1 * t135 * t16438 - 35.0 / 72.0 * t9782;
    (t16411, t16412, t16416, t16419, t16420, t16429, t16433, t16438, t16442)
}
