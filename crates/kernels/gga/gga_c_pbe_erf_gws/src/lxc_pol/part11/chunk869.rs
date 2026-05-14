//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 869/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk869<F: Float>(t5975: F, t992: F, t1964: F, t2519: F, t11274: F, t475: F, t1076: F, t169: F, t301: F, t922: F, t1368: F, t285: F, t3013: F, t1114: F, t19817: F, t19905: F) -> (F, F, F, F, F, F, F) {
    let t26437 = t992 * t5975;
    let t26439 = t2519 * t1964;
    let t26470 = t475 * t11274;
    let t26477 = t169 * t922 * t1076 * t301;
    let t26480 = t3013 * t1368 * t285;
    let t26755 = t1114 * t19817;
    let t26958 = t1114 * t19905;
    (t26437, t26439, t26470, t26477, t26480, t26755, t26958)
}
