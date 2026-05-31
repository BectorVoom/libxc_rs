//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1243/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1243<F: Float>(t2416: F, t4182: F, t353: F, t859: F, t938: F, t14415: F, t51563: F, t14397: F, t2367: F, t14652: F, t4414: F, t14127: F, t2503: F) -> (F, F, F, F, F, F) {
    let t53614 = t2416 * t4182;
    let t53617 = t859 * t353 * t53614 * t938;
    let t53625 = t51563 * t14415;
    let t53626 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t53625;
    let t53629 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t2367 * t14397;
    let t53636 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t4414 * t14652;
    let t53645 = t14127 * t2503;
    (t53614, t53617, t53626, t53629, t53636, t53645)
}
