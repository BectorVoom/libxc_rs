//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2692/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2692(t54811: f64, t119: f64, t12407: f64, t12429: f64, t1315: f64, t16242: f64, t16248: f64, t16265: f64, t16364: f64, t16383: f64, t210: f64, t3803: f64, t3805: f64, t3851: f64, t3856: f64, t40443: f64, t40449: f64, t5248: f64, t53856: f64, t54786: f64, t54787: f64, t54793: f64, t54801: f64) -> f64 {
    let t54812 = 119.0_f64 / 2304.0_f64 * t54811;
    let t54813 = -t12429 * t16265 / 1024.0_f64 + t3803 * t3805 * t16242 * t12407 / 256.0_f64 + t12429 * t16248 / 256.0_f64 + t54786 + 7.0_f64 / 48.0_f64 * t54787 - t1315 * t210 * t119 * t53856 / 48.0_f64 - 595.0_f64 / 10368.0_f64 * t54793 + 119.0_f64 / 4608.0_f64 * t40443 + t40449 - t3803 * t5248 * t16242 * t3851 / 1024.0_f64 - 7.0_f64 / 384.0_f64 * t54801 + t12429 * t16383 / 256.0_f64 + t3803 * t3805 * t16364 * t3856 / 256.0_f64 + t54812;
    t54813
}
