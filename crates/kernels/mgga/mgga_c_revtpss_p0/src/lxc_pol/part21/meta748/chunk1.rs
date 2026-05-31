//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2624/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2624<F: Float>(t48287: F, t39807: F, t39813: F, t47059: F, t47063: F, t47067: F, t47070: F, t47072: F, t47076: F, t48275: F, t48278: F, t48279: F, t48281: F, t48283: F, t48284: F, t48286: F) -> (F, F) {
    let t48288 = F::cast_from(24.0_f64) * t48287;
    let t48289 = t47059 + t48275 + t39807 - t39813 + t47063 + t47067 - t48278 - t47070 - t47072 + t48279 - t48281 - t48283 - t47076 - t48284 + t48286 + t48288;
    (t48288, t48289)
}
