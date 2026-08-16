//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 411/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk411(t2697: f64, t849: f64, t1891: f64, t241: f64, t67: f64, t2379: f64, t820: f64, t2553: f64, t847: f64, t249: f64, t2571: f64, t2602: f64, t2603: f64, t2606: f64, t2610: f64, t2614: f64, t2618: f64, t2621: f64, t2623: f64, t2630: f64, t2635: f64, t2640: f64, t2643: f64, t2649: f64, t2681: f64, t2686: f64, t2695: f64, t787: f64, t817: f64, t831: f64, t843: f64) -> (f64, f64, f64) {
    let t2698 = t2697 * t849;
    let t2700 = t241 * t1891;
    let t2701 = t2700 * t67;
    let t2703 = t2701 * t820 * t2379;
    let t2707 = t847 * t820 * t2553;
    let t2710 = t2602 + 7.0_f64 / 72.0_f64 * t2603 + t2571 * t2606 / 16.0_f64 - t787 * t2610 / 48.0_f64 + t2614 * t249 / 3072.0_f64 - t2618 * t831 / 1536.0_f64 - 7.0_f64 / 2304.0_f64 * t2621 - t2623 * t849 / 384.0_f64 + t2630 * t2635 / 1536.0_f64 + 7.0_f64 / 2304.0_f64 * t2640 + t2643 * t2649 / 384.0_f64 - t817 * t2681 / 3072.0_f64 - t817 * t2686 / 3072.0_f64 + t2695 + 7.0_f64 / 576.0_f64 * t2698 + 5.0_f64 / 768.0_f64 * t843 * t2703 - t843 * t2707 / 768.0_f64;
    (t2703, t2707, t2710)
}
