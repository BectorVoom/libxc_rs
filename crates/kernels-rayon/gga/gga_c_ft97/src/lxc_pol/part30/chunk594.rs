//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 594/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk594(t232: f64, t27557: f64, t27561: f64, t27596: f64, t6014: f64, t25: f64, t6776: f64, t3762: f64, t1095: f64, t24389: f64, t13580: f64, t1113: f64, t202: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27686 = t232 * t27557;
    let t27689 = t232 * t27561;
    let t27692 = t6014 * t27596;
    let t27695 = t6776 * t25;
    let t27696 = t27695 * t3762;
    let t27699 = t24389 * t1095;
    let t27700 = t13580 * t27699;
    let t27703 = t202 * t1113;
    (t27686, t27689, t27692, t27696, t27700, t27703)
}
