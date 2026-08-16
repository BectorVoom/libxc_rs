//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 330/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk330(t1651: f64, t378: f64, t92: f64, t1639: f64, t1640: f64, t1645: f64, t1649: f64) -> (f64, f64, f64) {
    let t1652 = t378 * t1651;
    let t1653 = t92 * t1652;
    let t1655 = t1639 + 2.0_f64 / 9.0_f64 * t1640 - 2.0_f64 / 9.0_f64 * t1645 + 2.0_f64 / 3.0_f64 * t1649 - t1653 / 3.0_f64;
    (t1652, t1653, t1655)
}
