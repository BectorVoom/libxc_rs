//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta693 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2509;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2510;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2511;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2512;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2513;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2514;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2515;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2516;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta693<F: Float>(t13062: F, t225: F, t13378: F, t193: F, t2379: F, t4331: F, t591: F, t2394: F, t4344: F, t4339: F, t13574: F, t690: F, t13577: F, t13568: F, t13583: F, t13586: F, t13571: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t47609, t47618, t47645, t47676, t47705) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2509::<F>(t13062, t225, t13378, t193, t2379, t4331, t591, t2394, t4344);
        let t47707 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2510::<F>(t2394, t4339);
        let t47709 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2511::<F>(t13574, t690);
        let t47711 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2512::<F>(t13577, t690);
        let t47713 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2513::<F>(t13568, t690);
        let t47715 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2514::<F>(t13583, t690);
        let t47717 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2515::<F>(t13586, t690);
        let t47724 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2516::<F>(t13571, t690);
    (t47609, t47618, t47645, t47676, t47705, t47707, t47709, t47711, t47713, t47715, t47717, t47724)
}
