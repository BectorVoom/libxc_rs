//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta91 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk596;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk597;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk598;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk599;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk600;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta91<F: Float>(t1911: F, t858: F, t1884: F, t1903: F, t259: F, t855: F, t870: F, t25: F, t1877: F, t335: F, t371: F, t202: F, t193: F, t28: F, t1268: F, t1873: F, t191: F, t513: F, t192: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t1912 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk596::<F>(t1911, t858);
        let t1914 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk597::<F>(t1884, t1903, t1912, t259, t855);
        let t1915 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk598::<F>(t1914, t870);
        let (t1918, t1932) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk599::<F>(t1915, t25, t1877, t335, t371);
        let (t1962, t1964, t1971, t1979, t1982, t1983) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk600::<F>(t1914, t202, t193, t870, t1915, t28, t1877, t1268, t1873, t191, t513, t192);
    (t1912, t1914, t1915, t1918, t1932, t1962, t1964, t1971, t1979, t1982, t1983)
}
