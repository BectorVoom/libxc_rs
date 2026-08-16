//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1806;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1807;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1808;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1809;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1810;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta368(t13532: f64, t2768: f64, t123: f64, t13559: f64, t882: f64, t13542: f64, t13546: f64, t10296: f64, t10298: f64, t10302: f64, t13567: f64, t13569: f64, t13572: f64, t13575: f64, t1540: f64, t2394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13577, t13578) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1806(t13532, t2768, t123);
        let (t13580, t13581) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1807(t13559, t882, t123);
        let (t13583, t13584) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1808(t13542, t882, t123);
        let (t13586, t13587) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1809(t13546, t882, t123);
        let (t13592, t13598) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1810(t10296, t10298, t10302, t13567, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t1540, t2394);
    (t13577, t13578, t13580, t13581, t13583, t13584, t13586, t13587, t13592, t13598)
}
