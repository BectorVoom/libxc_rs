//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 579/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk579(t1096: f64, t684: f64, t24278: f64, t420: f64, t704: f64, t679: f64, t992: f64, t689: f64, t17864: f64, t6023: f64, t3766: f64, t6042: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27533 = t1096 * t684;
    let t27534 = t24278 * t27533;
    let t27537 = t420 * t704;
    let t27538 = t992 * t679;
    let t27539 = t27538 * t689;
    let t27540 = t27537 * t27539;
    let t27543 = t6023 * t17864;
    let t27546 = t3766 * t6042;
    (t27533, t27534, t27539, t27540, t27543, t27546)
}
