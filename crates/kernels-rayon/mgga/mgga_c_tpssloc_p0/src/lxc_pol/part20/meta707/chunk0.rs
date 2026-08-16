//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2698/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2698(t1338: f64, t16413: f64, t12168: f64, t12181: f64, t12238: f64, t1332: f64, t1336: f64, t1352: f64, t1380: f64, t1381: f64, t16052: f64, t16055: f64, t16060: f64, t16206: f64, t16414: f64, t1825: f64, t1840: f64, t3901: f64, t3907: f64, t40479: f64, t5234: f64, t5348: f64, t53909: f64, t54527: f64) -> f64 {
    let t55039 = t1338 * t16413;
    let t55059 = -t12168 * t1336 * t5348 - 3.0_f64 * t1336 * t1352 * t55039 - t1336 * t1380 * t54527 - 3.0_f64 * t1336 * t16206 * t3901 - t1336 * t1825 * t40479 - 3.0_f64 * t12181 * t5234 + t12238 * t1840 + 3.0_f64 * t1332 * t16414 - 3.0_f64 * t1381 * t53909 + 18.0_f64 * t16052 * t16055 - 3.0_f64 * t16060 * t3907;
    t55059
}
