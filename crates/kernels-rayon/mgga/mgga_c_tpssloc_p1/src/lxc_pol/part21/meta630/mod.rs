//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta630 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2412;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2413;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta630(t2606: f64, t41008: f64, t782: f64, t9558: f64, t2617: f64, t9600: f64, t849: f64, t2642: f64, t9612: f64, t786: f64, t9569: f64, t805: f64, t2610: f64, t9541: f64, t222: f64, t39934: f64, t9637: f64, t2691: f64, t812: f64, t815: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41009, t41011, t41052, t41053, t41063, t41083, t41084) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2412(t2606, t41008, t782, t9558, t2617, t9600, t849, t2642, t9612, t786, t9569, t805);
        let (t41086, t41096, t41107, t41115) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2413(t2610, t9541, t222, t39934, t2617, t9637, t2691, t812, t815);
    (t41009, t41011, t41052, t41053, t41063, t41083, t41084, t41086, t41096, t41107, t41115)
}
