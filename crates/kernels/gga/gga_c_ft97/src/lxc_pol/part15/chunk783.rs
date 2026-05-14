//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 783/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk783<F: Float>(t161: F, t38061: F, t89: F, t157: F, t40424: F, t605: F, t9114: F, t142: F, t7763: F, t342: F, t511: F, t8639: F, t7800: F, t10050: F, t257: F, t255: F) -> (F, F, F, F, F, F, F) {
    let t41093 = 280.0 / 243.0 * t89 * t38061 * t161;
    let t41251 = t40424 * t157;
    let t41269 = t9114 * t605;
    let t41318 = t142 * t7763;
    let t41328 = 5.0 / 54.0 * t342 * t8639 * t511;
    let t41349 = t142 * t7800;
    let t41408 = 1.0 / t10050 / t257;
    let t41409 = t255 * t41408;
    (t41093, t41251, t41269, t41318, t41328, t41349, t41409)
}
