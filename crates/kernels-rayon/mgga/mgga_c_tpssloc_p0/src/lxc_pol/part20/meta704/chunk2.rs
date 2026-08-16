//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2676/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2676(t12012: f64, t12147: f64, t12164: f64, t1347: f64, t1348: f64, t16176: f64, t16186: f64, t16196: f64, t16199: f64, t1819: f64, t1821: f64, t225: f64, t3839: f64, t3847: f64, t5272: f64, t5278: f64, t5279: f64, t5283: f64, t53856: f64, t54311: f64, t54377: f64, t54391: f64, t54415: f64, t54426: f64, t54440: f64, t54454: f64, t54479: f64, t54525: f64, t546: f64, t548: f64, t550: f64) -> f64 {
    let t54527 = (-12.0_f64 * t5278 * t5279 * t12012 + 9.0_f64 * t5272 * t3847 + 3.0_f64 * t546 * t1347 * t53856 + 3.0_f64 * t1819 * t12164 + 3.0_f64 * t12147 * t1821 - (t54311 + t54377 + t54391 + t54415 + t54426 + t54440 + t54454 + t54479) * t225 * t548 + 9.0_f64 * t16176 * t1348 + 9.0_f64 * t3839 * t5283 - 72.0_f64 * t16186 * t16196 - 36.0_f64 * t16186 * t16199 + t54525) * t550;
    t54527
}
