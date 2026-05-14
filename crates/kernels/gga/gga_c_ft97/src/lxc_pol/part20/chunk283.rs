//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 283/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk283<F: Float>(t192: F, t2373: F, t2506: F, t2459: F, t743: F, t2481: F, t2482: F, t2484: F, t2489: F, t2494: F, t2499: F, t2503: F, t462: F, t92: F) -> (F, F, F) {
    let t2508 = t192 * t2506 * t2373;
    let t2512 = t192 * t743 * t2459;
    let t2514 = t2481 + 2.0 / 9.0 * t2482 + 2.0 / 3.0 * t2484 - 2.0 / 9.0 * t462 * t2489 + 2.0 / 3.0 * t462 * t2494 + 2.0 / 3.0 * t462 * t2499 - t462 * t2503 / 3.0 + 2.0 * t92 * t2508 - t92 * t2512;
    (t2508, t2512, t2514)
}
