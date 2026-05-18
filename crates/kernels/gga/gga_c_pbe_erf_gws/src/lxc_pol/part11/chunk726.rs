//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 726/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk726<F: Float>(t11190: F, t672: F, t3459: F, t679: F, t230: F, t11159: F, t164: F, t331: F, t3379: F, t551: F, t553: F, t3380: F, t547: F) -> (F, F, F, F, F, F, F) {
    let t11191 = t11190 * t672;
    let t11229 = t3459 * t679;
    let t11231 = t3459 * t230;
    let t11250 = t11159 * t164;
    let t11262 = t331 * t3379;
    let t11264 = t11262 * t551 * t553;
    let t11268 = t3380 * t547;
    (t11191, t11229, t11231, t11250, t11262, t11264, t11268)
}
