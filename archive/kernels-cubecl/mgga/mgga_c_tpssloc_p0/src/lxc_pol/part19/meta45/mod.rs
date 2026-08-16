//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta45 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk304;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk305;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk306;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk307;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta45<F: Float>(t894: F, t896: F, t880: F, t273: F, t241: F, t697: F, t281: F, t283: F, t340: F, t884: F, t136: F, t886: F, t290: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t897, t899, t901, t902, t904, t906, t907, t908) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk304::<F>(t894, t896, t880, t273, t241, t697, t281, t283, t340);
        let (t909, t910, t912) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk305::<F>(t884, t908, t136, t886, t897, t899, t902, t907);
        let t913 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk306::<F>(t290);
        let t914 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk307::<F>(t912, t913);
    (t897, t901, t902, t904, t906, t908, t909, t910, t912, t913, t914)
}
