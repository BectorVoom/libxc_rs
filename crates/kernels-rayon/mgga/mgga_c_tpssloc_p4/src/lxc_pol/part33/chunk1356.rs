//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1356/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1356(t100431: f64, t100436: f64, t1058: f64, t1060: f64, t106058: f64, t14608: f64, t1599: f64, t1945: f64, t1949: f64, t21118: f64, t21130: f64, t21134: f64, t21446: f64, t21594: f64, t23696: f64, t28597: f64, t28630: f64, t3200: f64, t3201: f64, t5838: f64, t6687: f64, t6784: f64, t6785: f64, t7614: f64, t82676: f64) -> f64 {
    let t106407 = t1058 * t1945 * t21594 * t1060 - 0.24674011002723396548e-1_f64 * t6687 * t1599 * t28630 - 0.24674011002723396548e-1_f64 * t6687 * t5838 * t7614 - 0.82246703342411321825e-2_f64 * t6687 * t21446 * t1949 - 3.0_f64 * t14608 * t28597 - 0.82246703342411321826e-2_f64 * t100431 + 0.27415567780803773942e-2_f64 * t100436 + 0.8529287754027840782e-2_f64 * t6687 * t82676 * t6785 * t21130 - 0.21932454224643019154e-1_f64 * t6687 * t23696 * t6785 * t21118 + 0.27415567780803773942e-2_f64 * t6687 * t6784 * t6785 * t21134 - 3.0_f64 * t3200 * t106058 * t3201;
    t106407
}
