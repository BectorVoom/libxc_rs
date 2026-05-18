//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 183/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk183<F: Float>(t40: F, t461: F, t1: F, t60: F, t119: F, t155: F, t84: F, t75: F) -> (F, F, F, F, F) {
    let t462 = t40 * t461;
    let t465 = t60 * t1;
    let t467 = t119 * t155 * t84;
    let t468 = t465 * t467;
    let t469 = F::new(0.18311555036753159941e-3) * t468;
    let t470 = t60 * t75;
    (t462, t465, t467, t469, t470)
}
