//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2681/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2681<F: Float>(t10013: F, t14224: F, t2782: F, t48073: F, t543: F, t4100: F, t47364: F, t47369: F, t47375: F, t47379: F, t47381: F, t47387: F, t47389: F, t47391: F, t47395: F) -> F {
    let t49296 = t2782 * t10013 * t14224;
    let t49306 = t48073 * t543;
    let t49308 = t2782 * t4100 * t49306;
    let t49310 = F::cast_from(0.32927245914677557992e-1_f64) * t49296 - F::cast_from(0.34697458558045176417e-2_f64) * t47364 - F::cast_from(0.9757440539382783019e-2_f64) * t47369 - F::cast_from(0.58544643236296698114e-1_f64) * t47375 + F::cast_from(0.58544643236296698114e-1_f64) * t47379 - F::cast_from(0.33133632253434461091e-3_f64) * t47381 - F::cast_from(0.54878743191129263322e-2_f64) * t47387 - F::cast_from(0.51220160311720645767e-1_f64) * t47389 + F::cast_from(0.19514881078765566038e-2_f64) * t47391 + F::cast_from(0.16463622957338778996e-1_f64) * t49308 - t47395;
    t49310
}
