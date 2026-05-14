//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 320/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk320<F: Float>(t2666: F, t2771: F, t2498: F, t848: F, t2502: F, t2: F, t2680: F) -> (F, F, F, F) {
    let t2772 = t2771 * t2666;
    let t2775 = t848 * t2498;
    let t2778 = t848 * t2502;
    let t2781 = t2680 * t2;
    (t2772, t2775, t2778, t2781)
}
