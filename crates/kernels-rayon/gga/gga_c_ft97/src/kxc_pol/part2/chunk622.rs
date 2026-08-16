//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 622/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk622(t1557: f64, t81: f64, t1559: f64, t1570: f64, t1528: f64, t1580: f64, t1755: f64, t72: f64, t1526: f64, t1527: f64, t1538: f64, t1565: f64, t1761: f64, t3088: f64, t342: f64, t343: f64, t7704: f64, t7707: f64, t7710: f64) -> f64 {
    let t7712 = t81 * t1557;
    let t7713 = t7712 * t1559;
    let t7720 = t81 * t1570;
    let t7721 = t7720 * t1559;
    let t7725 = t1528 * t1580;
    let t7729 = t72 * t1755;
    let t7733 = t1538 + t1761 + t7704 - t7707 / 18.0_f64 - t7710 / 6.0_f64 - t1526 * t3088 * t7713 / 9.0_f64 - t1526 * t1527 * t1565 / 6.0_f64 + t1526 * t1527 * t7721 / 6.0_f64 - t1526 * t1527 * t7725 / 12.0_f64 - t342 * t343 * t7729 / 4.0_f64;
    t7733
}
