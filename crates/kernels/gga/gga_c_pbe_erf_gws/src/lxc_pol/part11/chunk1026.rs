//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1026/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1026<F: Float>(t12323: F, t331: F, t551: F, t553: F, t145: F, t164: F, t12891: F, t547: F, t12882: F, t163: F, t169: F, t299: F) -> (F, F, F, F, F) {
    let t42244 = t331 * t12323 * t551 * t553;
    let t42251 = t145 * t12323;
    let t42252 = t42251 * t164;
    let t42265 = t12891 * t547;
    let t42272 = t169 * t299 * t12882 * t163;
    (t42244, t42251, t42252, t42265, t42272)
}
