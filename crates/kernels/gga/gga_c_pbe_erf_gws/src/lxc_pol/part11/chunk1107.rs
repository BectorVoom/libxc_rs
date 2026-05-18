//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1107/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1107<F: Float>(t40761: F, t40764: F, t47695: F, t47699: F, t47701: F, t47706: F, t47707: F, t47711: F, t47715: F, t47719: F, t47723: F, t40766: F) -> (F, F, F, F) {
    let t47724 = F::new(32.0) / F::new(15.0) * t40761;
    let t47725 = F::new(32.0) / F::new(45.0) * t40764;
    let t47726 = -t47695 - t47699 + t47701 + t47706 - t47707 + t47711 + t47715 - t47719 + t47723 + t47724 + t47725;
    let t47727 = F::new(32.0) / F::new(15.0) * t40766;
    (t47724, t47725, t47726, t47727)
}
