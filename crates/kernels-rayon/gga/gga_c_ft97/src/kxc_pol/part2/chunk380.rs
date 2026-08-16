//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 380/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk380(t139: f64, t2030: f64, t527: f64, t129: f64, t39: f64, t11: f64, t1689: f64, t1691: f64, t1696: f64, t1354: f64, t542: f64, t1702: f64, t554: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2031 = t139 * t2030;
    let t2032 = t527 * t2031;
    let t2034 = t129 * t39;
    let t2035 = t1689 * t11;
    let t2036 = t2034 * t2035;
    let t2037 = t1691 * t1696;
    let t2038 = t2037 * t139;
    let t2043 = t542 * t1354;
    let t2044 = t1702 * t554;
    (t2032, t2035, t2036, t2037, t2038, t2043, t2044)
}
