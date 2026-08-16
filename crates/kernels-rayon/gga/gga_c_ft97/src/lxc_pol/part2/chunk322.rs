//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 322/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk322(t1609: f64, t77: f64, t373: f64, t1608: f64, t384: f64, t6: f64, t51: f64, t53: f64, t397: f64, t371: f64, t409: f64, t428: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1610 = t1609 * t1609;
    let t1611 = t1610 * t1610;
    let t1613 = t1611 * t1611;
    let t1614 = t1613 * t1611 * t1609;
    let t1615 = t77 * t1614;
    let t1616 = t1615 * t373;
    let t1617 = t1608 * t1616;
    let t1618 = t384 * t6;
    let t1619 = t51 * t53;
    let t1620 = t1619 * t397;
    let t1621 = t1618 * t1620;
    let t1624 = t371 * t409;
    let t1625 = t384 * t428;
    (t1613, t1614, t1615, t1616, t1617, t1619, t1620, t1621, t1624, t1625)
}
