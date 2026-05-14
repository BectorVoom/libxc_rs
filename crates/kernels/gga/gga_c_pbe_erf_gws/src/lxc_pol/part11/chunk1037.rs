//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1037/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1037<F: Float>(t31492: F, t47725: F, t47727: F, t47728: F, t47729: F, t47730: F, t47731: F, t47732: F, t47737: F, t47741: F, t47746: F, t47751: F, t47752: F, t47753: F, t47760: F, t47761: F, t47762: F, t47765: F, t47769: F, t47772: F, t47775: F, t47776: F) -> (F, F) {
    let t48642 = t47725 - t47727 + t47728 + t47729 + t47730 - t47731 - t47732 - t47737 + t47741 + 16.0 * t31492 + t47746;
    let t48645 = t47751 + t47752 + t47753 + t47760 - t47761 + t47762 + t47765 - t47769 + t47772 + t47775 + t47776;
    (t48642, t48645)
}
