//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1110/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1110<F: Float>(t40824: F, t23817: F, t47727: F, t47728: F, t47729: F, t47730: F, t47731: F, t47732: F, t47737: F, t47741: F, t47746: F, t47751: F) -> (F, F, F) {
    let t47752 = F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t40824;
    let t47753 = F::cast_from(128.0_f64) / F::cast_from(1215.0_f64) * t23817;
    let t47754 = -t47727 + t47728 + t47729 + t47730 - t47731 - t47732 - t47737 + t47741 + t47746 + t47751 + t47752 + t47753;
    (t47752, t47753, t47754)
}
