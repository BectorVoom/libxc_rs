//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta92 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk597;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk598;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk599;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk600;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk601;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk602;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk603;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta92(t1902: f64, t218: f64, t1894: f64, t252: f64, t214: f64, t1880: f64, t235: f64, t226: f64, t858: f64, t1884: f64, t259: f64, t855: f64, t870: f64, t25: f64, t1877: f64, t337: f64, t38: f64, t1887: f64, t225: f64, t381: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1903, t1905, t1906, t1907, t1909) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk597(t1902, t218, t1894, t252, t214, t1880, t235);
        let t1911 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk598(t1907, t1909, t226);
        let t1912 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk599(t1911, t858);
        let t1914 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk600(t1884, t1903, t1912, t259, t855);
        let t1915 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk601(t1914, t870);
        let (t1918, t1919, t1920) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk602(t1915, t25, t1877, t337, t38, t1887);
        let t1921 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk603(t225, t381);
    (t1903, t1905, t1906, t1909, t1911, t1912, t1914, t1915, t1918, t1919, t1920, t1921)
}
