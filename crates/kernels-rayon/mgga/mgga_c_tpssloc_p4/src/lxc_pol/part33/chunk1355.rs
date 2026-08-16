//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1355/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1355(t100378: f64, t100390: f64, t100399: f64, t106355: f64, t1920: f64, t1948: f64, t1953: f64, t21481: f64, t21614: f64, t21617: f64, t23327: f64, t25470: f64, t28609: f64, t345: f64, t353: f64, t383: f64, t6797: f64, t6799: f64, t6800: f64, t82799: f64, t89431: f64, t89449: f64) -> f64 {
    let t106375 = t353 * t383 * t106355 + t82799 - 0.16449340668482264365e-1_f64 * t23327 * t25470 * t28609 + 0.24674011002723396548e-1_f64 * t6797 * t6799 * t21617 * t6800 - 0.54831135561607547883e-2_f64 * t100378 + 0.82246703342411321825e-2_f64 * t1920 * t345 * t1948 * t21614 - 0.54831135561607547884e-2_f64 * t89431 - 0.82246703342411321826e-2_f64 * t100390 + 0.54831135561607547884e-2_f64 * t89449 + 0.54831135561607547883e-2_f64 * t100399 + t21481 * t1953;
    t106375
}
