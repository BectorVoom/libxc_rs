//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta317 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1682;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1683;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1684;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1685;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta317(t11721: f64, t6739: f64, t10471: f64, t3502: f64, t11712: f64, t3508: f64, t11707: f64, t3609: f64, t3623: f64, t1209: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11883, t11887, t11888) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1682(t11721, t6739, t10471, t3502, t11712);
        let (t11889, t11904) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1683(t3508, t6739, t11707, t3609);
        let t11907 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1684(t11707, t3623);
        let (t11913, t11914) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1685(t10471, t1209, t11712);
    (t11883, t11887, t11888, t11889, t11904, t11907, t11913, t11914)
}
