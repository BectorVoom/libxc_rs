//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1334/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1334<F: Float>(t361: F, t57321: F, t13917: F, t3223: F, t12014: F, t13919: F, t1173: F, t12166: F, t3824: F, t898: F, t14682: F, t3989: F, t50912: F) -> (F, F, F, F, F) {
    let t57432 = t361 * t57321;
    let t57434 = t13917 * t57432 * t3223;
    let t57441 = t13917 * t13919 * t12014;
    let t57449 = t1173 * t12166;
    let t57451 = t898 * t3824;
    let t57454 = t3989 * t14682 * t57451 * t50912;
    (t57434, t57441, t57449, t57451, t57454)
}
