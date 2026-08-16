//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1224/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1224(t38282: f64, t38298: f64, t38303: f64, t38308: f64, t38312: f64, t40659: f64, t40662: f64, t40666: f64, t40670: f64, t40672: f64, t40679: f64, t40683: f64, t40686: f64, t40690: f64, t40694: f64) -> f64 {
    let t40695 = -t38282 + 0.34200192530023447503e-6_f64 * t40659 + t40662 - t40666 + t40670 - 0.35220688045884876043e-2_f64 * t40672 - t38298 - 0.14408463291498358381e-2_f64 * t38303 + 0.36021158228745895953e-3_f64 * t38308 + t38312 - t40679 - t40683 - t40686 - t40690 + t40694;
    t40695
}
