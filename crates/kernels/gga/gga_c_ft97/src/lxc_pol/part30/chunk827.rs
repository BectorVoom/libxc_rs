//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 827/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk827<F: Float>(t35318: F, t9942: F, t1434: F, t193: F, t2506: F, t35323: F, t1154: F, t7484: F, t743: F, t6109: F, t33508: F, t33513: F, t35312: F, t35316: F, t35321: F, t35326: F, t35330: F, t35334: F) -> (F, F, F, F, F, F, F, F) {
    let t35336 = t9942 * t35318;
    let t35338 = t1434 * t193 * t35336;
    let t35339 = t2506 * t35323;
    let t35341 = t1434 * t193 * t35339;
    let t35343 = t7484 * t1154;
    let t35344 = t743 * t35343;
    let t35346 = t6109 * t193 * t35344;
    let t35348 = t35312 / F::cast_from(2.0_f64) + t33508 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t35316 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t35321 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t35326 - t35330 / F::cast_from(6.0_f64) - t33513 - t35334 / F::cast_from(9.0_f64) - t35338 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t35341 + t35346 / F::cast_from(12.0_f64);
    (t35336, t35338, t35339, t35341, t35343, t35344, t35346, t35348)
}
