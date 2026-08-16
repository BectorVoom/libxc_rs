//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 869/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk869(t120: f64, t1570: f64, t16: f64, t8946: f64, t2252: f64, t341: f64, t37820: f64, t23: f64, t32905: f64, t153: f64, t1984: f64, t22: f64, t36452: f64, t37991: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39931 = t120 * t1570;
    let t39942 = t8946 * t16;
    let t39976 = t341 * t2252;
    let t40033 = 0.4939111192043895748e-1_f64 * t37820;
    let t40266 = t23 * t32905;
    let t40280 = 1.0_f64 / t153 / t37991 / t22 / t1984 / t36452 / 96.0_f64;
    (t39931, t39942, t39976, t40033, t40266, t40280)
}
