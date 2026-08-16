//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 334/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk334(t1693: f64, t45: f64, t55: f64, t1692: f64, t1690: f64, t12: f64, t51: f64) -> (f64, f64, f64) {
    let t1696 = 1.0_f64 / t45 / t1693 / t55;
    let t1697 = t1692 * t1696;
    let t1698 = t1690 * t1697;
    let t1701 = t51 * t12;
    (t1696, t1698, t1701)
}
