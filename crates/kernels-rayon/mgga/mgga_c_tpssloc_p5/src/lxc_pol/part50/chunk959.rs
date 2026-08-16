//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 959/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk959(t25717: f64, t6784: f64, t2770: f64, t381: f64, t3961: f64, t25510: f64, t23613: f64, t7603: f64, t1003: f64, t1058: f64, t23327: f64, t23346: f64, t23712: f64, t25429: f64, t25563: f64, t25568: f64, t25706: f64, t25708: f64, t25714: f64, t3186: f64, t353: f64, t6680: f64, t6687: f64, t7604: f64, t7615: f64, t7622: f64) -> f64 {
    let t25718 = t6784 * t25717;
    let t25721 = t381 * t2770;
    let t25722 = t25721 * t3961;
    let t25723 = t25510 * t25722;
    let t25726 = t23613 * t7603;
    let t25729 = -0.73108180748810063845e-2_f64 * t23346 * t7604 + 0.91385225936012579807e-3_f64 * t25563 - 0.21932454224643019153e-1_f64 * t6680 * t7615 + t1058 * t25568 + t1003 * t7622 + t353 * t25706 + 2.0_f64 * t3186 * t25708 + 0.91385225936012579807e-3_f64 * t23712 - 0.82246703342411321825e-2_f64 * t6687 * t25714 + 0.27415567780803773942e-2_f64 * t6687 * t25718 + 0.36554090374405031923e-2_f64 * t25429 * t25723 - 0.27415567780803773942e-2_f64 * t23327 * t25726;
    t25729
}
