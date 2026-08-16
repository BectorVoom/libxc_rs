//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 770/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk770(t1516: f64, t2621: f64, t2623: f64, t2640: f64, t2643: f64, t2695: f64, t2698: f64, t4191: f64, t4236: f64, t4240: f64, t4250: f64, t4253: f64, t4257: f64, t4261: f64, t817: f64, t843: f64) -> f64 {
    let t4264 = t2643 * t4191 / 768.0_f64 - t817 * t4236 / 3072.0_f64 - t2643 * t4240 / 3072.0_f64 - 7.0_f64 / 4608.0_f64 * t2621 + 7.0_f64 / 4608.0_f64 * t2640 + t2695 + 7.0_f64 / 1152.0_f64 * t2698 - t2623 * t1516 / 768.0_f64 + t2643 * t4250 / 768.0_f64 + 7.0_f64 / 1152.0_f64 * t4253 + 5.0_f64 / 768.0_f64 * t843 * t4257 - t843 * t4261 / 768.0_f64;
    t4264
}
