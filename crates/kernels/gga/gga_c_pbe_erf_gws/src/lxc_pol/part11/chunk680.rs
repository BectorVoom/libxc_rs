//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 680/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk680<F: Float>(t11159: F, t242: F, t168: F, t3609: F, t703: F, t163: F, t169: F, t299: F, t3569: F, t1: F, t3: F, t3379: F, t672: F, t3459: F, t679: F, t230: F) -> (F, F, F, F, F, F, F) {
    let t11160 = t11159 * t242;
    let t11166 = t168 * t703 * t3609;
    let t11187 = t169 * t299 * t3569 * t163;
    let t11190 = t3379 * t1 * t3;
    let t11191 = t11190 * t672;
    let t11229 = t3459 * t679;
    let t11231 = t3459 * t230;
    (t11160, t11166, t11187, t11190, t11191, t11229, t11231)
}
