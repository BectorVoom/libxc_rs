//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta93 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk597;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk598;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk599;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta93(t1915: f64, t25: f64, t1877: f64, t335: f64, t371: f64, t1914: f64, t202: f64, t193: f64, t870: f64, t28: f64, t1268: f64, t1873: f64, t191: f64, t513: f64, t192: f64, t209: f64, t540: f64, t1878: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1918, t1932) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk597(t1915, t25, t1877, t335, t371);
        let (t1962, t1964, t1971, t1979, t1982, t1983) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk598(t1914, t202, t193, t870, t1915, t28, t1877, t1268, t1873, t191, t513, t192);
        let (t1984, t1985) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk599(t209, t540, t1878);
    (t1918, t1932, t1962, t1964, t1971, t1979, t1982, t1983, t1984, t1985)
}
