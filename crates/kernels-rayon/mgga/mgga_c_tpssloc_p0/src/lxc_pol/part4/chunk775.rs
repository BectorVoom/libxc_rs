//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 775/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk775(t232: f64, t5584: f64, t819: f64, t820: f64, t2701: f64, t5527: f64, t5544: f64, t847: f64, t1512: f64, t1516: f64, t249: f64, t2571: f64, t2602: f64, t2630: f64, t2643: f64, t2695: f64, t4152: f64, t4167: f64, t4170: f64, t4172: f64, t4187: f64, t4253: f64, t5568: f64, t5572: f64, t5576: f64, t5587: f64, t5593: f64, t5614: f64, t787: f64, t817: f64, t843: f64) -> (f64, f64, f64, f64, f64) {
    let t5617 = t5584 * t232;
    let t5619 = t819 * t820 * t5617;
    let t5624 = t2701 * t820 * t5527;
    let t5628 = t847 * t820 * t5544;
    let t5631 = t2602 + 7.0_f64 / 72.0_f64 * t4152 + t2571 * t5568 / 16.0_f64 - t787 * t5572 / 48.0_f64 + t5576 * t249 / 3072.0_f64 - t4167 * t1512 / 1536.0_f64 - 7.0_f64 / 2304.0_f64 * t4170 - t4172 * t1516 / 384.0_f64 + t2630 * t5587 / 1536.0_f64 + 7.0_f64 / 2304.0_f64 * t4187 + t2643 * t5593 / 384.0_f64 - t817 * t5614 / 3072.0_f64 - t817 * t5619 / 3072.0_f64 + t2695 + 7.0_f64 / 576.0_f64 * t4253 + 5.0_f64 / 768.0_f64 * t843 * t5624 - t843 * t5628 / 768.0_f64;
    (t5617, t5619, t5624, t5628, t5631)
}
