//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1249/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1249(t10771: f64, t713: f64, t3550: f64, t5802: f64, t683: f64, t7285: f64, t10841: f64, t10878: f64, t1108: f64, t17519: f64, t17521: f64, t20834: f64, t21090: f64, t21179: f64, t26048: f64, t26224: f64, t26357: f64, t2801: f64, t2815: f64, t2820: f64, t2849: f64, t3581: f64, t3605: f64, t5825: f64, t5871: f64, t702: f64, t723: f64, t7324: f64, t7478: f64, t7486: f64, t9410: f64, t9413: f64, t9416: f64, t9429: f64, t9452: f64, t9499: f64) -> (f64, f64) {
    let t30685 = t10771 * t713;
    let t30697 = 0.1551780387578202009e4_f64 * t5802 * t3550 * t7285 * t683;
    let t30700 = 0.6207121550312808036e4_f64 * t5871 * t9429 * t2815 + 0.19964560303604640732e6_f64 * t17519 * t10841 * t17521 * t702 - 6.0_f64 * t26357 * t2801 + 0.96491876992155210402e2_f64 * t26224 * t2820 + 18.0_f64 * t7324 * t9410 - 12.0_f64 * t7486 * t9413 - 0.57895126195293126241e3_f64 * t21090 * t9416 + 0.96491876992155210402e2_f64 * t21179 * t3581 + 6.0_f64 * t5825 * t10878 + 0.5848223622634646207e0_f64 * t30685 * t723 + 0.17544670867903938621e1_f64 * t26048 * t1108 + 0.17544670867903938621e1_f64 * t9499 * t2849 + 0.17544670867903938621e1_f64 * t7478 * t3605 - t30697 - 0.31168546390226634766e3_f64 * t20834 * t9452;
    (t30697, t30700)
}
