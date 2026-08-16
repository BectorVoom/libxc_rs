//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta92 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk597;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk598;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk599;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk600;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk601;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk602;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk603;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta92<F: Float>(t1893: F, t1895: F, t235: F, t59: F, t226: F, t249: F, t1888: F, t218: F, t1894: F, t252: F, t214: F, t1880: F, t858: F, t1884: F, t259: F, t855: F, t870: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1896, t1898) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk597::<F>(t1893, t1895, t235, t59);
        let (t1899, t1902) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk598::<F>(t1898, t226, t249, t1888, t1896);
        let (t1903, t1905, t1906, t1907, t1909) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk599::<F>(t1902, t218, t1894, t252, t214, t1880, t235);
        let t1911 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk600::<F>(t1907, t1909, t226);
        let t1912 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk601::<F>(t1911, t858);
        let t1914 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk602::<F>(t1884, t1903, t1912, t259, t855);
        let t1915 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk603::<F>(t1914, t870);
    (t1898, t1899, t1902, t1903, t1905, t1906, t1909, t1911, t1912, t1914, t1915)
}
