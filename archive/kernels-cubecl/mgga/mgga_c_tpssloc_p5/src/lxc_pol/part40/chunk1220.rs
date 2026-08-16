//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1220/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1220<F: Float>(t19356: F, t19444: F, t12568: F, t12571: F, t1437: F, t19297: F, t19299: F, t19310: F, t19313: F, t19318: F, t2235: F, t2240: F, t3953: F, t3958: F, t4021: F, t5389: F, t5445: F, t605: F, t645: F, t86: F, t9231: F, t9239: F) -> F {
    let t19445 = t19356 + t19444;
    let t19448 = -F::cast_from(8.0_f64) * t12568 * t1437 + F::cast_from(40.0_f64) * t12571 * t3958 + t19297 * t86 - F::cast_from(4.0_f64) * t19299 * t645 - F::cast_from(120.0_f64) * t19310 * t9239 + F::cast_from(40.0_f64) * t19313 * t2240 + F::cast_from(20.0_f64) * t19318 * t2240 - F::cast_from(4.0_f64) * t19445 * t605 - F::cast_from(4.0_f64) * t2235 * t5445 - F::cast_from(8.0_f64) * t3953 * t4021 + F::cast_from(20.0_f64) * t5389 * t9231;
    t19448
}
