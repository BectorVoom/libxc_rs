//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 983/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk983(t40367: f64, t40403: f64, t40446: f64, t40486: f64, t579: f64, t91: f64, t2120: f64, t2086: f64, t3000: f64, t520: f64, t89: f64, t1975: f64, t7773: f64) -> (f64, f64, f64, f64) {
    let t40490 = t91 * t579 * (t40367 + t40403 + t40446 + t40486);
    let t40492 = t2120 * t2120;
    let t40494 = t91 * t2086 * t40492;
    let t40497 = t89 * t3000 * t520;
    let t40500 = t89 * t7773 * t1975;
    (t40490, t40494, t40497, t40500)
}
