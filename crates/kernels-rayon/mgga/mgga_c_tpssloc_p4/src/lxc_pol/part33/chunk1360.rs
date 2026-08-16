//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1360/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1360(t1599: f64, t17588: f64, t1920: f64, t1956: f64, t21134: f64, t21614: f64, t225: f64, t25810: f64, t28491: f64, t28515: f64, t345: f64, t387: f64, t5844: f64, t6687: f64, t6689: f64, t6690: f64, t70978: f64, t7553: f64, t7561: f64, t7565: f64, t7600: f64, t88138: f64, t89672: f64, t99210: f64, t99214: f64, t99895: f64, t99948: f64, t99956: f64) -> f64 {
    let t106526 = 0.82246703342411321826e-2_f64 * t6687 * t25810 * t28515 + 0.82246703342411321826e-2_f64 * t6687 * t99214 * t7553 + 0.36554090374405031922e-2_f64 * t99948 + 0.27415567780803773942e-2_f64 * t6687 * t6689 * t6690 * t21134 + 0.10966227112321509577e-1_f64 * t6687 * t88138 * t28491 - 0.54831135561607547883e-2_f64 * t99956 + 0.24674011002723396548e-1_f64 * t6687 * t1599 * t99210 - 0.24674011002723396548e-1_f64 * t6687 * t5844 * t7561 + 0.82246703342411321825e-2_f64 * t1920 * t345 * t21614 * t225 * t387 - t70978 * t1956 + 0.54831135561607547884e-2_f64 * t89672 - 0.24674011002723396548e-1_f64 * t6687 * t99895 * t7565 + 12.0_f64 * t17588 * t7600;
    t106526
}
