//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 150/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk150<F: Float>(t10: F, t144: F, t351: F, t143: F, t358: F, t363: F, t356: F, t89: F, t142: F) -> (F, F, F, F, F, F, F) {
    let t517 = t10 * t351 * t144;
    let t518 = t517 / 18.0;
    let t519 = t143 * t358;
    let t520 = t519 * t363;
    let t522 = t89 * t356 * t520;
    let t524 = t142 * t142;
    let t525 = 1.0 / t524;
    (t517, t518, t519, t520, t522, t524, t525)
}
