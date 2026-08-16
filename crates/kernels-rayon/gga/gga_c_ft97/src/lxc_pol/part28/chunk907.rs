//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 907/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk907(t37482: f64, t383: f64, t11120: f64, t3018: f64, t62: f64, t8417: f64, t971: f64, t1851: f64, t3170: f64, t110: f64, t38477: f64, t38463: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45499 = t37482 * t383;
    let t45500 = t45499 * t11120;
    let t45572 = t3018 * t62;
    let t46565 = t971 * t8417;
    let t46727 = t3170 * t1851;
    let t46874 = t38477 * t110;
    let t46881 = t38463 * t110;
    (t45499, t45500, t45572, t46565, t46727, t46874, t46881)
}
