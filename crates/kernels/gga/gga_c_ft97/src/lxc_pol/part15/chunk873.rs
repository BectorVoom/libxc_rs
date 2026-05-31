//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 873/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk873<F: Float>(t605: F, t9114: F, t142: F, t7763: F, t342: F, t511: F, t8639: F, t7800: F, t10050: F, t257: F, t255: F, t2346: F, t2359: F) -> (F, F, F, F, F, F) {
    let t41269 = t9114 * t605;
    let t41318 = t142 * t7763;
    let t41328 = F::cast_from(5.0_f64) / F::cast_from(54.0_f64) * t342 * t8639 * t511;
    let t41349 = t142 * t7800;
    let t41408 = F::cast_from(1.0_f64) / t10050 / t257;
    let t41409 = t255 * t41408;
    let t41446 = F::cast_from(1.0_f64) / t2346 / t2359;
    (t41269, t41318, t41328, t41349, t41409, t41446)
}
