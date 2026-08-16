//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2316/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2316(t23384: f64, t28660: f64, t28614: f64, t362: f64, t5914: f64, t14618: f64, t23327: f64, t23670: f64, t23685: f64, t25568: f64, t25708: f64, t25713: f64, t28605: f64, t28631: f64, t4669: f64, t5685: f64, t5903: f64, t6680: f64, t6687: f64, t6784: f64, t6813: f64, t7603: f64, t884: f64, t89532: f64, t89546: f64, t99921: f64) -> f64 {
    let t100431 = t23384 * t28660;
    let t100436 = t23384 * t28614;
    let t100449 = t362 * t5914;
    let t100459 = -0.27415567780803773942e-2_f64 * t100431 - 0.82246703342411321825e-2_f64 * t6687 * t99921 * t25713 + 0.91385225936012579807e-3_f64 * t100436 + 0.27415567780803773942e-2_f64 * t6687 * t6784 * t23685 * t5685 + t5903 * t6813 + 4.0_f64 * t14618 * t25708 - 0.21932454224643019153e-1_f64 * t6680 * t28631 + 2.0_f64 * t4669 * t25568 + 0.27415567780803773942e-2_f64 * t6687 * t6784 * t100449 * t884 + 0.43864908449286038307e-1_f64 * t23670 * t28605 - 0.54831135561607547884e-2_f64 * t23327 * t89532 * t7603 + t89546;
    t100459
}
