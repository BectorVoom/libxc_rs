//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta91 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk567;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk568;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk569;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta91(t1898: f64, t226: f64, t249: f64, t1894: f64, t252: f64, t214: f64, t1880: f64, t335: f64, t371: f64, t191: f64, t513: f64, t192: f64, t209: f64, t540: f64, t1878: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1899, t1900, t1905, t1906, t1907, t1932) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk567(t1898, t226, t249, t1894, t252, t214, t1880, t335, t371);
        let (t1982, t1983) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk568(t191, t513, t192);
        let (t1984, t1985) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk569(t209, t540, t1878);
    (t1899, t1900, t1905, t1906, t1907, t1932, t1982, t1983, t1984, t1985)
}
