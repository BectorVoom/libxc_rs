//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 706/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk706<F: Float>(t6110: F, t825: F, t822: F, t2418: F, t338: F, t892: F, t2220: F, t939: F, t2359: F, t2373: F, t2379: F, t2384: F, t2388: F, t2408: F, t335: F, t4385: F, t4459: F, t4464: F, t4467: F, t4469: F, t4475: F, t4477: F, t4484: F, t4487: F, t4489: F, t4493: F, t4496: F, t6107: F, t833: F) -> (F, F, F, F, F) {
    let t6111 = t6110 * t825;
    let t6112 = t822 * t6111;
    let t6116 = t338 * t892 * t2418;
    let t6120 = t338 * t2220 * t939;
    let t6123 = -t2359 * t4459 / 32.0 - t2359 * t4464 / 96.0 + 7.0 / 48.0 * t4467 + 7.0 / 24.0 * t4469 - t2388 * t2373 / 16.0 - 7.0 / 96.0 * t4475 - 7.0 / 96.0 * t4477 - t2384 * t2379 / 32.0 + t4385 * t4484 / 32.0 + 35.0 / 144.0 * t4487 + 7.0 / 48.0 * t4489 + t2408 * t4493 / 16.0 + 7.0 / 48.0 * t4496 + t6107 * t833 / 96.0 + t6112 * t833 / 96.0 + t335 * t6116 / 16.0 - t335 * t6120 / 32.0;
    (t6111, t6112, t6116, t6120, t6123)
}
