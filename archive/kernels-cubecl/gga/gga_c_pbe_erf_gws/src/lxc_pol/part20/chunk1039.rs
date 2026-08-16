//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1039/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1039<F: Float>(t11514: F, t3140: F, t3235: F, t2170: F, t3814: F, t8840: F, t2168: F, t11478: F, t6287: F, t3138: F, t8884: F, t8890: F) -> (F, F, F, F, F, F) {
    let t11640 = t3235 * t11514 * t3140;
    let t11644 = t2170 * t8840 * t3814;
    let t11646 = t2168 * t11644 / F::cast_from(24.0_f64);
    let t11648 = t2170 * t11478 * t6287;
    let t11650 = t3138 * t11648 / F::cast_from(24.0_f64);
    let t11651 = t8884 * t8890;
    (t11640, t11644, t11646, t11648, t11650, t11651)
}
