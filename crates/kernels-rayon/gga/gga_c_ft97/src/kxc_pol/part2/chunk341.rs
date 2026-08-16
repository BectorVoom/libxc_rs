//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 341/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk341(t1755: f64, t370: f64, t27: f64, t89: f64, t1545: f64, t1549: f64, t1552: f64, t1562: f64, t1567: f64, t1574: f64, t1583: f64, t1591: f64) -> (f64, f64, f64) {
    let t1756 = t370 * t1755;
    let t1758 = t89 * t27 * t1756;
    let t1760 = t1545 + t1549 + t1552 - t1562 / 27.0_f64 + t1567 / 9.0_f64 + t1574 / 9.0_f64 - t1583 / 18.0_f64 + t1591 / 3.0_f64 - t1758 / 6.0_f64;
    (t1756, t1758, t1760)
}
