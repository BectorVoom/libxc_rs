//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1366/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1366(t3032: f64, t3131: f64, t23614: f64, t82431: f64, t11023: f64, t11030: f64, t11047: f64, t23346: f64, t23601: f64, t23603: f64, t23604: f64, t23606: f64, t23613: f64, t23670: f64, t23674: f64, t23680: f64, t23685: f64, t23693: f64, t23696: f64, t23697: f64, t23698: f64, t25429: f64, t2771: f64, t6687: f64, t6797: f64, t6799: f64, t6800: f64, t6802: f64, t82513: f64, t82515: f64, t82516: f64, t82527: f64, t82534: f64, t82539: f64, t82541: f64) -> f64 {
    let t82542 = t3032 * t3131;
    let t82555 = t82431 * t23614;
    let t82561 = -0.10966227112321509577e-1_f64 * t25429 * t23613 * t23697 + 0.49348022005446793095e-1_f64 * t82513 * t82515 * t11047 * t82516 + 0.24674011002723396548e-1_f64 * t6797 * t6799 * t11030 * t6800 + 0.24125699647107321069e0_f64 * t82527 * t6802 - 0.65797362673929057459e-1_f64 * t23670 * t23674 - 0.13159472534785811492e0_f64 * t82534 * t23680 + 0.65797362673929057459e-1_f64 * t82534 * t23606 + 0.16449340668482264365e-1_f64 * t82539 - 0.49348022005446793095e-1_f64 * t82513 * t82541 * t11047 * t82542 - 0.24674011002723396548e-1_f64 * t23601 * t23603 * t11023 * t23604 - 0.21932454224643019154e-1_f64 * t23346 * t23693 - 0.29243272299524025538e-1_f64 * t23346 * t23698 - 0.54831135561607547883e-2_f64 * t82555 + 0.10966227112321509577e-1_f64 * t6687 * t23696 * t23685 * t2771;
    t82561
}
