//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 449/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk449(t235: f64, t2718: f64, t231: f64, t159: f64, t243: f64, t216: f64, t2712: f64, t785: f64, t225: f64, t826: f64, t849: f64, t820: f64, t823: f64, t843: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2719 = t2718 * t235;
    let t2723 = t231 * t231;
    let t2729 = t159 * t243;
    let t2730 = t216 * t2729;
    let t2735 = t2712 * t785;
    let t2736 = t2735 * t225;
    let t2737 = t849 * t826;
    let t2739 = 0.25410001404642664112e-5_f64 * t2736 * t2737;
    let t2741 = t820 * t823 * t843;
    (t2719, t2723, t2730, t2735, t2736, t2739, t2741)
}
