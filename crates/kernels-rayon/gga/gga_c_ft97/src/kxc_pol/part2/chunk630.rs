//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 630/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk630(t1608: f64, t7999: f64, t1689: f64, t1691: f64, t1696: f64, t1609: f64, t77: f64, t1593: f64, t1615: f64, t1630: f64, t1619: f64, t1681: f64) -> (f64, f64, f64, f64, f64) {
    let t8000 = t1608 * t7999;
    let t8002 = t1689 * t1691;
    let t8003 = t8002 * t1696;
    let t8007 = t77 * t1609;
    let t8008 = t8007 * t1593;
    let t8009 = t1608 * t8008;
    let t8014 = t1615 * t1630;
    let t8015 = t1608 * t8014;
    let t8018 = t1619 * t1681;
    (t8000, t8003, t8009, t8015, t8018)
}
