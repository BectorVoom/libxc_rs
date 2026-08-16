//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2350/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2350<F: Float>(t2108: F, t2240: F, t5392: F, t1409: F, t605: F, t1410: F, t2110: F, t24520: F, t24526: F, t26009: F, t27972: F, t27976: F, t6492: F, t7246: F, t9239: F, t96502: F, t96506: F, t96517: F, t96521: F, t96553: F, t96556: F) -> F {
    let t104907 = t2240 * t5392 * t2108;
    let t104911 = t605 * t1409 * t2108;
    let t104916 = F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t24520 * t27972 + F::cast_from(20.0_f64) * t9239 * t1410 * t2108 * t26009 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t24526 * t27972 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7246 * t96502 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7246 * t96506 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t24520 * t27976 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t24526 * t27976 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7246 * t96517 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7246 * t96521 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t104907 * t6492 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t104911 * t96553 + t96556 * t2110 / F::cast_from(3.0_f64);
    t104916
}
