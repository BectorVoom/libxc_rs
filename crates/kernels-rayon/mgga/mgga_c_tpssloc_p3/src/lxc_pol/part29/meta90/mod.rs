//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta90 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk589;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk590;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk591;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk592;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk593;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk594;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk595;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta90(t201: f64, t243: f64, t598: f64, t213: f64, t225: f64, t234: f64, t236: f64, t235: f64, t59: f64, t226: f64, t249: f64, t1888: f64, t218: f64, t252: f64, t214: f64, t1880: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1891 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk589(t201, t243);
        let (t1892, t1893, t1894) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk590(t1891, t598, t213, t225, t234);
        let t1895 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk591(t1894, t236);
        let (t1896, t1898) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk592(t1893, t1895, t235, t59);
        let (t1899, t1902) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk593(t1898, t226, t249, t1888, t1896);
        let (t1903, t1905, t1906, t1907, t1909) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk594(t1902, t218, t1894, t252, t214, t1880, t235);
        let t1911 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk595(t1907, t1909, t226);
    (t1891, t1892, t1894, t1895, t1898, t1899, t1902, t1903, t1905, t1906, t1909, t1911)
}
