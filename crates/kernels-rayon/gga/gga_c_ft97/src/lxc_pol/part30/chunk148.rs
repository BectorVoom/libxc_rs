//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 148/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk148(t170: f64, t328: f64, t626: f64, t327: f64, t668: f64, t231: f64, t505: f64, t322: f64, t70: f64) -> (f64, f64, f64, f64) {
    let t892 = t170 * t626 * t328 / 6.0_f64;
    let t893 = t327 * t668;
    let t895 = t231 * t893 * t505;
    let t898 = t70 * t322;
    (t892, t893, t895, t898)
}
