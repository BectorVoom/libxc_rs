//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 332/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk332(t45: f64, t55: f64, t389: f64, t44: f64, t54: f64, t52: f64) -> (f64, f64, f64) {
    let t1675 = 1.0_f64 / t45 / t55;
    let t1679 = t55 * t389;
    let t1681 = 1.0_f64 / t44 / t1679;
    let t1682 = t54 * t1681;
    let t1683 = t52 * t1682;
    (t1675, t1681, t1683)
}
