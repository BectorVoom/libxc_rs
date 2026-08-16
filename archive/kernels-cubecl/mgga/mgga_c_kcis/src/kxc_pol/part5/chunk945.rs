//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 945/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk945<F: Float>(t209: F, t698: F, t9215: F, t2399: F, t2406: F, t2412: F, t4879: F, t63: F, t696: F, t702: F, t75: F, t9195: F, t9206: F, t9211: F) -> F {
    let t9217 = t209 * t698 * t9215;
    let t9220 = -F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t63 * t4879 * t75 - F::cast_from(35.0_f64) / F::cast_from(144.0_f64) * t9195 * t702 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t2399 * t2406 + F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t2399 * t2412 - t696 * t9206 / F::cast_from(16.0_f64) + t696 * t9211 / F::cast_from(16.0_f64) - t696 * t9217 / F::cast_from(96.0_f64);
    t9220
}
