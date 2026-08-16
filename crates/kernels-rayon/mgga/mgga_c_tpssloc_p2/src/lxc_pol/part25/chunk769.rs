//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 769/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk769(t232: f64, t9957: f64, t819: f64, t820: f64, t2571: f64, t2618: f64, t2643: f64, t2649: f64, t2686: f64, t817: f64, t9642: f64, t9649: f64, t9653: f64, t9657: f64, t9663: f64, t9668: f64, t9672: f64, t9675: f64, t9679: f64) -> (f64, f64, f64) {
    let t9958 = t9957 * t232;
    let t9960 = t819 * t820 * t9958;
    let t9963 = t9642 * t2649 / 128.0_f64 - 5.0_f64 / 256.0_f64 * t2643 * t9649 + t2643 * t9653 / 256.0_f64 + 3.0_f64 / 16.0_f64 * t2571 * t9657 - t817 * t9663 / 3072.0_f64 - 7.0_f64 / 768.0_f64 * t9668 - 119.0_f64 / 4608.0_f64 * t9672 + 7.0_f64 / 768.0_f64 * t9675 - t2618 * t2686 / 1024.0_f64 + 7.0_f64 / 1536.0_f64 * t9679 - t817 * t9960 / 3072.0_f64;
    (t9958, t9960, t9963)
}
