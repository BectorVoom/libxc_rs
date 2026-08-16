//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 449/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk449<F: Float>(t238: F, t2455: F, t695: F, t224: F, t2381: F, t2384: F, t2387: F, t2389: F, t2396: F, t2419: F, t2422: F, t2429: F, t678: F) -> F {
    let t239 = F::cast_from(0.1e-59_f64) < t238;
    let t2456 = t695 * t2455;
    let t2459 = piecewise3::<F>(t239, F::cast_from(0.67598802253579164263e-4_f64) * t2381 * t2384 + F::cast_from(0.23254900946437792e-1_f64) * t2387 * t2389 + F::cast_from(0.23254900946437792e-2_f64) * t678 * t2396 - F::cast_from(0.11627450473218896e-1_f64) * t678 * t2419 + F::cast_from(0.19365723406274399941e-3_f64) * t678 * t2422 + F::cast_from(2.0_f64) * t224 * t2429 - t224 * t2456, F::cast_from(0.0_f64));
    t2459
}
