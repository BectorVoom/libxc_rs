//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 665/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk665<F: Float>(t24: F, t9017: F, t9236: F, t2118: F, t458: F, t462: F, t92: F, t9205: F, t9207: F, t9209: F, t9211: F, t9214: F, t9218: F, t9221: F, t9225: F, t9230: F, t9233: F) -> (F, F) {
    let t9238 = t24 * t9236 * t9017;
    let t9241 = t458 * t2118;
    let t9242 = t9205 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t9207 - F::new(2.0) * t9209 - F::new(2.0) * t462 * t9211 + F::new(2.0) * t462 * t9214 - F::new(2.0) * t462 * t9218 - F::new(2.0) * t462 * t9221 - F::new(10.0) / F::new(27.0) * t462 * t9225 + F::new(6.0) * t462 * t9230 - t462 * t9233 / F::new(3.0) - F::new(6.0) * t92 * t9238 + t9241;
    (t9238, t9242)
}
