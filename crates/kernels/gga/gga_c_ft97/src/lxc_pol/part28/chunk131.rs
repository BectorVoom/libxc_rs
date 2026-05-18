//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 131/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk131<F: Float>(t363: F, t142: F, t72: F, t149: F, t342: F, t343: F, t10: F, t144: F, t351: F, t143: F, t358: F, t356: F, t89: F) -> (F, F, F, F, F, F, F, F) {
    let t505 = -t363;
    let t511 = t72 * t142;
    let t515 = t149 - t342 * t343 * t511 / F::new(4.0);
    let t517 = t10 * t351 * t144;
    let t518 = t517 / F::new(18.0);
    let t519 = t143 * t358;
    let t520 = t519 * t363;
    let t522 = t89 * t356 * t520;
    (t505, t511, t515, t517, t518, t519, t520, t522)
}
