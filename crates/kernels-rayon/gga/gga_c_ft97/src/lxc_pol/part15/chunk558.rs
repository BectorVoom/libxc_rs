//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 558/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk558(t1711: f64, t371: f64, t407: f64, t66: f64, t2247: f64, t47: f64, t68: f64, t72: f64, t1557: f64, t422: f64, t1736: f64, t7800: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8042 = t371 * t1711;
    let t8050 = t407 * t407;
    let t8051 = 1.0_f64 / t8050;
    let t8052 = t66 * t8051;
    let t8076 = t47 * t2247;
    let t8078 = t68 * t8076 * t72;
    let t8079 = 0.70937342644032921812e-2_f64 * t8078;
    let t8088 = t422 * t1557;
    let t8101 = t1736 * t7800;
    (t8042, t8050, t8051, t8052, t8078, t8079, t8088, t8101)
}
