//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 349/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk349(t1544: f64, t1548: f64, t1551: f64, t1562: f64, t1567: f64, t1574: f64, t1583: f64, t1591: f64, t1758: f64, t1769: f64, t1810: f64) -> (f64, f64) {
    let t1812 = 4.0_f64 / 9.0_f64 * t1544;
    let t1820 = -t1769 / 4.0_f64 + t1810 / 2.0_f64 + t1812 + 2.0_f64 / 9.0_f64 * t1548 + 2.0_f64 / 3.0_f64 * t1551 - 2.0_f64 / 9.0_f64 * t1562 + 2.0_f64 / 3.0_f64 * t1567 + 2.0_f64 / 3.0_f64 * t1574 - t1583 / 3.0_f64 + 2.0_f64 * t1591 - t1758;
    (t1812, t1820)
}
