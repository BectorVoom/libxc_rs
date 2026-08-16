//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 514/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk514(t2999: f64, t355: f64, t1597: f64, t383: f64, t1594: f64, t63: f64, t66: f64) -> (f64, f64, f64) {
    let t3000 = t2999 * t355;
    let t3018 = t383 * t1597;
    let t3019 = t1594 * t3018;
    let t3020 = t63 * t66;
    (t3000, t3019, t3020)
}
