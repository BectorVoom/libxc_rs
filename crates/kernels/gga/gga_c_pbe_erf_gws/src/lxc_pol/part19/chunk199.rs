//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 199/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk199<F: Float>(t550: F, t551: F, t553: F, t163: F, t169: F, t234: F, t299: F, t172: F, t181: F, t184: F) -> (F, F, F, F) {
    let t555 = F::new(0.19753890328909480882e-2) * t550 * t551 * t553;
    let t559 = F::new(0.89806755076909568204e-2) * t169 * t299 * t234 * t163;
    let t560 = t172 * t181;
    let t561 = t560 * t184;
    (t555, t559, t560, t561)
}
