//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1372/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1372(t1009: f64, t343: f64, t25490: f64, t6746: f64, t884: f64, t10309: f64, t10327: f64, t11007: f64, t11047: f64, t11077: f64, t1920: f64, t1948: f64, t1949: f64, t23327: f64, t23601: f64, t23633: f64, t23636: f64, t23679: f64, t23696: f64, t345: f64, t6687: f64, t6785: f64, t6786: f64, t6797: f64, t6799: f64, t6800: f64, t82513: f64, t82605: f64, t82618: f64, t82620: f64, t82625: f64, t82629: f64, t82633: f64, t82635: f64, t82637: f64, t82638: f64, t82643: f64, t82653: f64) -> f64 {
    let t82654 = t343 * t1009;
    let t82655 = t82654 * t25490;
    let t82657 = t82655 * t884 * t6746;
    let t82660 = -0.82246703342411321826e-2_f64 * t82605 - 0.82246703342411321825e-2_f64 * t6687 * t10327 * t1949 + 0.24674011002723396548e-1_f64 * t6797 * t6799 * t11077 * t6800 + 0.82246703342411321825e-2_f64 * t1920 * t345 * t1948 * t11007 - 0.16449340668482264365e-1_f64 * t82618 - 0.49348022005446793095e-1_f64 * t23601 * t82620 * t23679 + 0.16449340668482264365e-1_f64 * t23633 * t82625 * t23636 + 0.43864908449286038307e-1_f64 * t82629 + 0.54831135561607547884e-2_f64 * t82633 - 0.18277045187202515961e-2_f64 * t82635 + 0.82246703342411321825e-2_f64 * t82513 * t82637 * t11047 * t82638 - 0.82246703342411321826e-2_f64 * t23327 * t82643 * t6786 - 0.21932454224643019154e-1_f64 * t6687 * t23696 * t6785 * t10309 - 0.16449340668482264365e-1_f64 * t82653 * t82657;
    t82660
}
