//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 933/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk933<F: Float>(t141370: F, t141384: F, t151001: F, t151004: F, t151008: F, t151011: F, t151014: F, t151017: F, t151020: F, t151025: F, t151027: F, t151030: F, t151033: F, t151035: F, t151040: F, t151043: F) -> (F,) {
    let t151344 = 4.0 / 3.0 * t151001 - 4.0 / 9.0 * t151004 + 2.0 / 3.0 * t151008 - t151011 / 9.0 + t151014 / 27.0 + t151017 / 18.0 + 2.0 / 3.0 * t151020 + t151025 / 3.0 - t151027 / 9.0 - 8.0 / 9.0 * t151030 - t151033 / 3.0 - t151035 / 36.0 + t151040 / 12.0 - 4.0 / 9.0 * t151043 - 4.0 / 27.0 * t141370 + t141384 / 27.0;
    (t151344,)
}
