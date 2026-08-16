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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2509;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2510;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2511;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2512;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2513;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2514;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2515;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2516;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta693(t13062: f64, t225: f64, t13378: f64, t193: f64, t2379: f64, t4331: f64, t591: f64, t2394: f64, t4344: f64, t4339: f64, t13574: f64, t690: f64, t13577: f64, t13568: f64, t13583: f64, t13586: f64, t13571: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47609, t47618, t47645, t47676, t47705) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2509(t13062, t225, t13378, t193, t2379, t4331, t591, t2394, t4344);
        let t47707 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2510(t2394, t4339);
        let t47709 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2511(t13574, t690);
        let t47711 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2512(t13577, t690);
        let t47713 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2513(t13568, t690);
        let t47715 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2514(t13583, t690);
        let t47717 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2515(t13586, t690);
        let t47724 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2516(t13571, t690);
    (t47609, t47618, t47645, t47676, t47705, t47707, t47709, t47711, t47713, t47715, t47717, t47724)
}
