//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2284/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2284(t24574: f64, t27412: f64, t5052: f64, t7299: f64, t14972: f64, t15359: f64, t15790: f64, t1716: f64, t2123: f64, t24596: f64, t24601: f64, t24617: f64, t27381: f64, t27396: f64, t27406: f64, t27549: f64, t27820: f64, t3243: f64, t3593: f64, t4930: f64, t7283: f64, t7295: f64, t7302: f64, t7351: f64, t7392: f64, t85787: f64, t85789: f64, t86452: f64) -> f64 {
    let t94535 = 0.10966227112321509577e-1_f64 * t24574 * t27412;
    let t94558 = t7299 * t5052;
    let t94564 = -2.0_f64 * t14972 * t7392 + t94535 - 0.82246703342411321825e-2_f64 * t7283 * t15359 * t2123 - 0.16449340668482264365e-1_f64 * t7283 * t4930 * t7295 + 4.0_f64 * t7351 * t15790 - 0.43864908449286038306e-1_f64 * t27406 * t24617 - 0.36554090374405031923e-2_f64 * t27549 * t24601 * t27381 * t3243 - 0.36554090374405031923e-2_f64 * t27549 * t27820 * t24596 + 0.18277045187202515961e-2_f64 * t85787 + 0.82246703342411321825e-2_f64 * t7283 * t1716 * t86452 - 0.18277045187202515961e-2_f64 * t85789 - 0.16449340668482264365e-1_f64 * t7283 * t94558 * t7302 + 4.0_f64 * t3593 * t27396;
    t94564
}
