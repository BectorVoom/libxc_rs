//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2681/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2681(t10013: f64, t14224: f64, t2782: f64, t48073: f64, t543: f64, t4100: f64, t47364: f64, t47369: f64, t47375: f64, t47379: f64, t47381: f64, t47387: f64, t47389: f64, t47391: f64, t47395: f64) -> f64 {
    let t49296 = t2782 * t10013 * t14224;
    let t49306 = t48073 * t543;
    let t49308 = t2782 * t4100 * t49306;
    let t49310 = 0.32927245914677557992e-1_f64 * t49296 - 0.34697458558045176417e-2_f64 * t47364 - 0.9757440539382783019e-2_f64 * t47369 - 0.58544643236296698114e-1_f64 * t47375 + 0.58544643236296698114e-1_f64 * t47379 - 0.33133632253434461091e-3_f64 * t47381 - 0.54878743191129263322e-2_f64 * t47387 - 0.51220160311720645767e-1_f64 * t47389 + 0.19514881078765566038e-2_f64 * t47391 + 0.16463622957338778996e-1_f64 * t49308 - t47395;
    t49310
}
