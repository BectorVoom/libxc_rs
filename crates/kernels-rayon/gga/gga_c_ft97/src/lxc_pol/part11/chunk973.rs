//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 973/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk973(t27: f64, t40262: f64, t526: f64, t89: f64, t23: f64, t32905: f64, t1986: f64, t2999: f64, t559: f64, t153: f64, t1984: f64, t22: f64, t36452: f64, t37991: f64) -> (f64, f64, f64, f64, f64) {
    let t40265 = t89 * t27 * t526 * t40262;
    let t40266 = t23 * t32905;
    let t40267 = t1986 * t1986;
    let t40270 = t89 * t27 * t40266 * t40267;
    let t40273 = t89 * t2999 * t559;
    let t40280 = 1.0_f64 / t153 / t37991 / t22 / t1984 / t36452 / 96.0_f64;
    (t40265, t40267, t40270, t40273, t40280)
}
