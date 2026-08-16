//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1358/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1358(t106355: f64, t1603: f64, t17588: f64, t1922: f64, t21138: f64, t21446: f64, t21458: f64, t21663: f64, t25784: f64, t28593: f64, t28679: f64, t349: f64, t388: f64, t4557: f64, t5838: f64, t6687: f64, t6689: f64, t6690: f64, t6771: f64, t7561: f64, t7625: f64, t88882: f64, t99439: f64, t99864: f64) -> f64 {
    let t106460 = -t6771 * t21663 + 0.54831135561607547884e-2_f64 * t88882 - 0.82246703342411321826e-2_f64 * t99439 + 0.16449340668482264365e-1_f64 * t6687 * t6689 * t6690 * t21138 - 0.82246703342411321825e-2_f64 * t6687 * t21446 * t1922 - 0.24674011002723396548e-1_f64 * t6687 * t5838 * t7561 - 3.0_f64 * t4557 * t28679 - 6.0_f64 * t17588 * t7625 + 3.0_f64 * t1603 * t28593 * t388 + t349 * t106355 * t388 - 0.16449340668482264365e-1_f64 * t99864 + 0.24674011002723396548e-1_f64 * t6687 * t5838 * t25784 - 0.82246703342411321825e-2_f64 * t6687 * t21458 * t1922;
    t106460
}
