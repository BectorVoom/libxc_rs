//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1267/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1267<F: Float>(t2416: F, t4182: F, t353: F, t859: F, t938: F, t13917: F, t14424: F, t9551: F, t14415: F, t51563: F, t14397: F, t2367: F) -> (F, F, F, F, F) {
    let t53614 = t2416 * t4182;
    let t53617 = t859 * t353 * t53614 * t938;
    let t53623 = t13917 * t14424 * t9551;
    let t53625 = t51563 * t14415;
    let t53626 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t53625;
    let t53629 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t2367 * t14397;
    (t53614, t53617, t53623, t53626, t53629)
}
