//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 176/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk176<F: Float>(t353: F, t142: F, t72: F, t149: F, t342: F, t343: F, t10: F, t144: F, t351: F, t143: F, t358: F) -> (F, F, F, F, F, F, F, F) {
    let t490 = t353 / F::new(3.0);
    let t511 = t72 * t142;
    let t515 = t149 - t342 * t343 * t511 / F::new(4.0);
    let t517 = t10 * t351 * t144;
    let t518 = t517 / F::new(18.0);
    let t519 = t143 * t358;
    let t524 = t142 * t142;
    let t525 = F::new(1.0) / t524;
    (t490, t511, t515, t517, t518, t519, t524, t525)
}
