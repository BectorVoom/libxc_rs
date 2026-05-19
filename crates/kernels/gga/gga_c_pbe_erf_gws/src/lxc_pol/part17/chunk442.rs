//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 442/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk442<F: Float>(t1684: F, t174: F, t205: F, t332: F, t395: F, t628: F, t56: F, t641: F) -> (F, F, F, F, F) {
    let t1685 = F::new(8.0) / F::new(45.0) * t1684;
    let t1687 = t174 * t332 * t205;
    let t1688 = F::cast_from(0.47988888888888888889e-1_f64) * t1687;
    let t1689 = t395 * t628;
    let t1691 = t56 * t641;
    (t1685, t1687, t1688, t1689, t1691)
}
