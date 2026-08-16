//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta92 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk565;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk566;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk567;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta92<F: Float>(t1898: F, t226: F, t249: F, t1894: F, t252: F, t214: F, t1880: F, t335: F, t371: F, t191: F, t513: F, t192: F, t209: F, t540: F, t1878: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1899, t1900, t1905, t1906, t1907, t1932) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk565::<F>(t1898, t226, t249, t1894, t252, t214, t1880, t335, t371);
        let (t1982, t1983) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk566::<F>(t191, t513, t192);
        let (t1984, t1985) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk567::<F>(t209, t540, t1878);
    (t1899, t1900, t1905, t1906, t1907, t1932, t1982, t1983, t1984, t1985)
}
