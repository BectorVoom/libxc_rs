//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 331/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk331(t1655: f64, t35: f64, t374: f64, t1594: f64, t1632: f64, t401: f64, t38: f64, t78: f64, t388: f64, t66: f64, t408: f64, t428: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1656 = t1655 * t35;
    let t1657 = t374 * t1656;
    let t1660 = t1594 * t1632;
    let t1663 = t401 * t401;
    let t1664 = t38 * t1663;
    let t1665 = t1664 * t78;
    let t1669 = t388 * t66;
    let t1670 = t408 * t401;
    let t1671 = t1670 * t428;
    (t1657, t1660, t1663, t1664, t1665, t1669, t1671)
}
