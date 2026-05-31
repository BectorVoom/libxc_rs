//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 421/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk421<F: Float>(t2294: F, t637: F, t639: F, t2251: F, t2254: F, t2256: F, t2261: F, t2265: F, t2268: F, t2273: F, t2277: F, t2284: F, t631: F) -> (F, F) {
    let t2296 = t637 * t639 * t2294;
    let t2299 = -t2251 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2254 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2256 + t631 * t2261 / F::cast_from(18.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2265 * t2268 - t631 * t2273 / F::cast_from(3.0_f64) + t631 * t2277 / F::cast_from(6.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t631 * t2284 + t631 * t2296 / F::cast_from(2.0_f64);
    (t2296, t2299)
}
