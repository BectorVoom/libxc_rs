//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta596 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2059;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta596(t22998: f64, t23185: f64, t81914: f64, t6604: f64, t9971: f64, t23110: f64, t23180: f64, t206: f64, t22723: f64, t268: f64, t23186: f64, t23163: f64, t23165: f64, t1879: f64, t80845: f64, t1906: f64, t23229: f64, t81715: f64, t225: f64, t23226: f64, t23228: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82016, t82018, t82028, t82031, t82032, t82038) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2059(t22998, t23185, t81914, t6604, t9971, t23110, t23180, t206, t22723, t268, t23186, t23163);
        let (t82039, t82045, t82047, t82070, t82071, t82074) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2060(t23165, t82038, t1879, t80845, t1906, t23229, t81715, t225, t23226, t23228);
    (t82016, t82018, t82028, t82031, t82032, t82038, t82039, t82045, t82047, t82070, t82071, t82074)
}
