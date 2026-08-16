//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 392/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk392(t2999: f64, t355: f64, t375: f64, t89: f64, t943: f64, t1586: f64, t942: f64, t63: f64, t66: f64) -> (f64, f64, f64, f64) {
    let t3000 = t2999 * t355;
    let t3006 = t89 * t375 * t943;
    let t3013 = t1586 * t942;
    let t3020 = t63 * t66;
    (t3000, t3006, t3013, t3020)
}
