//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 448/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk448<F: Float>(t2334: F, t2338: F, t2341: F, t2352: F, t2357: F, t2364: F, t2368: F, t2376: F, t2462: F, t2478: F, t2516: F) -> (F, F) {
    let t2518 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2334;
    let t2526 = -t2478 / F::cast_from(4.0_f64) + t2516 / F::cast_from(2.0_f64) + t2518 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2338 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2341 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2352 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2357 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2364 - t2368 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) * t2376 - t2462;
    (t2518, t2526)
}
