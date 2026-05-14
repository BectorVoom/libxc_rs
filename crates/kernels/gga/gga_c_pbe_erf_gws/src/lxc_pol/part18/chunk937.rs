//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 937/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk937<F: Float>(t11514: F, t3140: F, t3235: F, t2170: F, t3814: F, t8840: F, t2168: F, t11478: F, t6287: F, t3138: F, t8884: F, t8890: F, t9665: F, t3257: F, t3803: F, t6355: F) -> (F, F, F, F, F, F, F, F) {
    let t11640 = t3235 * t11514 * t3140;
    let t11644 = t2170 * t8840 * t3814;
    let t11646 = t2168 * t11644 / 24.0;
    let t11648 = t2170 * t11478 * t6287;
    let t11650 = t3138 * t11648 / 24.0;
    let t11651 = t8884 * t8890;
    let t11652 = t9665 * t11651;
    let t11656 = t3257 * t3803 * t6355;
    (t11640, t11644, t11646, t11648, t11650, t11651, t11652, t11656)
}
