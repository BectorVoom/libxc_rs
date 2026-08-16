//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 874/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk874<F: Float>(t16739: F, t16669: F, t11: F, t16738: F, t16576: F, t39: F) -> (F, F, F, F, F) {
    let t16740 = F::cast_from(1.0_f64) / t16739;
    let t16741 = t16740 * t16669;
    let t16743 = t11 * t16738 * t16741;
    let t16745 = -t39 + t16576;
    let t16746 = F::cast_from(24.0_f64) * t16745;
    (t16740, t16741, t16743, t16745, t16746)
}
