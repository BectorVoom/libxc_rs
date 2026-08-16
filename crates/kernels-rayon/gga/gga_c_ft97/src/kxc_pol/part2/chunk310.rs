//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 310/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk310(t379: f64, t432: f64, t1564: f64, t446: f64, t21: f64, t357: f64) -> (f64, f64, f64, f64) {
    let t1565 = t379 * t432;
    let t1566 = t1564 * t1565;
    let t1567 = t446 * t1566;
    let t1569 = t357 * t21;
    let t1570 = 1.0_f64 / t1569;
    (t1565, t1566, t1567, t1570)
}
