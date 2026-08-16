//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 976/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk976(t7780: f64, t89: f64, t9055: f64, t1984: f64, t28: f64, t558: f64, t9007: f64, t376: f64, t9022: f64, t1979: f64, t7773: f64, t1965: f64, t37345: f64) -> (f64, f64, f64, f64, f64) {
    let t40301 = t89 * t7780 * t9055;
    let t40306 = t89 * t28 * t1984 * t9007 * t558;
    let t40309 = t89 * t376 * t9022;
    let t40312 = t89 * t7773 * t1979;
    let t40315 = t89 * t37345 * t1965;
    (t40301, t40306, t40309, t40312, t40315)
}
