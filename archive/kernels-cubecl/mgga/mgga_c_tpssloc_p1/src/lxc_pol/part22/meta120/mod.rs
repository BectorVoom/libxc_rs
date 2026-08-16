//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta120 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk810;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk811;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk812;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta120<F: Float>(t1227: F, t3523: F, t1009: F, t1190: F, t1011: F, t1212: F, t374: F, t486: F, t677: F, t485: F, t1203: F, t1222: F, t221: F, t3426: F, t456: F, t1197: F, t135: F, t1174: F, t1176: F, t3247: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3524, t3534, t3535, t3536) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk810::<F>(t1227, t3523, t1009, t1190, t1011, t1212);
        let t3540 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk811::<F>(t374, t486, t677);
        let (t3542, t3543, t3545, t3547, t3548, t3549, t3555) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk812::<F>(t3540, t485, t1203, t1222, t221, t3426, t456, t1197, t135, t1174, t1176, t3247);
    (t3524, t3534, t3535, t3536, t3540, t3542, t3543, t3545, t3547, t3548, t3549, t3555)
}
