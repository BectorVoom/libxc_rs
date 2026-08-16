//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1249/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1249<F: Float>(t10771: F, t713: F, t3550: F, t5802: F, t683: F, t7285: F, t10841: F, t10878: F, t1108: F, t17519: F, t17521: F, t20834: F, t21090: F, t21179: F, t26048: F, t26224: F, t26357: F, t2801: F, t2815: F, t2820: F, t2849: F, t3581: F, t3605: F, t5825: F, t5871: F, t702: F, t723: F, t7324: F, t7478: F, t7486: F, t9410: F, t9413: F, t9416: F, t9429: F, t9452: F, t9499: F) -> (F, F) {
    let t30685 = t10771 * t713;
    let t30697 = F::cast_from(0.1551780387578202009e4_f64) * t5802 * t3550 * t7285 * t683;
    let t30700 = F::cast_from(0.6207121550312808036e4_f64) * t5871 * t9429 * t2815 + F::cast_from(0.19964560303604640732e6_f64) * t17519 * t10841 * t17521 * t702 - F::cast_from(6.0_f64) * t26357 * t2801 + F::cast_from(0.96491876992155210402e2_f64) * t26224 * t2820 + F::cast_from(18.0_f64) * t7324 * t9410 - F::cast_from(12.0_f64) * t7486 * t9413 - F::cast_from(0.57895126195293126241e3_f64) * t21090 * t9416 + F::cast_from(0.96491876992155210402e2_f64) * t21179 * t3581 + F::cast_from(6.0_f64) * t5825 * t10878 + F::cast_from(0.5848223622634646207e0_f64) * t30685 * t723 + F::cast_from(0.17544670867903938621e1_f64) * t26048 * t1108 + F::cast_from(0.17544670867903938621e1_f64) * t9499 * t2849 + F::cast_from(0.17544670867903938621e1_f64) * t7478 * t3605 - t30697 - F::cast_from(0.31168546390226634766e3_f64) * t20834 * t9452;
    (t30697, t30700)
}
