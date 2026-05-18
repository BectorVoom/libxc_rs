//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 793/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk793<F: Float>(t209: F, t698: F, t9215: F, t2399: F, t2406: F, t2412: F, t4879: F, t63: F, t696: F, t702: F, t75: F, t9195: F, t9206: F, t9211: F) -> F {
    let t9217 = t209 * t698 * t9215;
    let t9220 = -F::new(455.0) / F::new(1296.0) * t63 * t4879 * t75 - F::new(35.0) / F::new(144.0) * t9195 * t702 - F::new(7.0) / F::new(48.0) * t2399 * t2406 + F::new(7.0) / F::new(96.0) * t2399 * t2412 - t696 * t9206 / F::new(16.0) + t696 * t9211 / F::new(16.0) - t696 * t9217 / F::new(96.0);
    t9220
}
