//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 565/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk565(t1663: f64, t37: f64, t78: f64, t1693: f64, t56: f64, t45: f64, t1690: f64, t1692: f64, t1632: f64, t401: f64, t1631: f64, t44: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7918 = t37 * t1663;
    let t7919 = t7918 * t78;
    let t7922 = t1693 * t56;
    let t7924 = 1.0_f64 / t45 / t7922;
    let t7926 = t1690 * t1692 * t7924;
    let t7929 = t1632 * t401;
    let t7930 = t1631 * t7929;
    let t7934 = 1.0_f64 / t44 / t1693;
    (t7918, t7919, t7924, t7926, t7929, t7930, t7934)
}
