//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta607 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1998;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1999;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta607(t131: f64, t845: f64, t23143: f64, t6649: f64, t6604: f64, t9971: f64, t206: f64, t22723: f64, t268: f64, t23186: f64, t23163: f64, t23165: f64, t1879: f64, t80845: f64, t1906: f64, t23229: f64, t81715: f64, t225: f64, t23228: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81982, t82011, t82018, t82031, t82032, t82038, t82039) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1998(t131, t845, t23143, t6649, t6604, t9971, t206, t22723, t268, t23186, t23163, t23165);
        let (t82045, t82047, t82070, t82074) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1999(t1879, t80845, t1906, t23229, t81715, t225, t23228);
    (t81982, t82011, t82018, t82031, t82032, t82038, t82039, t82045, t82047, t82070, t82074)
}
