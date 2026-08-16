//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2708/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2708<F: Float>(t1437: F, t19445: F, t20201: F, t20204: F, t20288: F, t2235: F, t2240: F, t39054: F, t39063: F, t3953: F, t4021: F, t5389: F, t5445: F, t605: F, t645: F, t75356: F, t75392: F, t75419: F, t75547: F, t9231: F, t9239: F) -> F {
    let t75552 = -F::cast_from(12.0_f64) * t3953 * t19445 - F::cast_from(120.0_f64) * t39054 * t20201 + F::cast_from(840.0_f64) * t39063 * t20201 * t645 - F::cast_from(360.0_f64) * t9239 * t5389 * t4021 + F::cast_from(60.0_f64) * t9231 * t20204 - F::cast_from(360.0_f64) * t9239 * t20204 * t645 + F::cast_from(60.0_f64) * t2240 * t4021 * t5445 + F::cast_from(60.0_f64) * t2240 * t1437 * t19445 - F::cast_from(4.0_f64) * t2235 * t20288 + F::cast_from(20.0_f64) * t2240 * t20288 * t645 - F::cast_from(4.0_f64) * t605 * (t75356 + t75392 + t75419 + t75547);
    t75552
}
