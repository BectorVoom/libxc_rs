//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 802/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk802(t10850: f64, t2917: f64, t9571: f64, t4334: f64, t9583: f64, t2413: f64, t904: f64, t2923: f64, t2951: f64, t684: f64, t230: f64, t2938: f64) -> (f64, f64, f64, f64, f64) {
    let t10852 = t2917 * t10850 * t9571;
    let t10855 = t4334 * t9583;
    let t10858 = t2413 * t904;
    let t10859 = t2923 * t10858;
    let t10861 = t684 * t2951;
    let t10862 = t2923 * t10861;
    let t10864 = t230 * t2938;
    (t10852, t10855, t10859, t10862, t10864)
}
