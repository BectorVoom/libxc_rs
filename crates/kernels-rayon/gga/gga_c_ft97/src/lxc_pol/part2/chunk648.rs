//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 648/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk648(t520: f64, t7773: f64, t89: f64, t1546: f64, t1979: f64, t1965: f64, t7780: f64, t1987: f64, t375: f64, t128: f64, t39: f64, t2035: f64) -> (f64, f64, f64, f64, f64) {
    let t8796 = t89 * t7773 * t520;
    let t8799 = t89 * t1546 * t1979;
    let t8802 = t89 * t7780 * t1965;
    let t8805 = t89 * t375 * t1987;
    let t8811 = t128 * t39;
    let t8812 = t8811 * t2035;
    (t8796, t8799, t8802, t8805, t8812)
}
