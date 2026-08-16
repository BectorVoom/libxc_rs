//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 647/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk647(t142: f64, t1557: f64, t1559: f64, t1570: f64, t1580: f64, t1943: f64, t2075: f64, t72: f64, t1526: f64, t1527: f64, t1953: f64, t1970: f64, t2081: f64, t3088: f64, t342: f64, t343: f64, t8759: f64, t8761: f64, t8764: f64) -> f64 {
    let t8766 = t142 * t1557;
    let t8767 = t8766 * t1559;
    let t8774 = t142 * t1570;
    let t8775 = t8774 * t1559;
    let t8779 = t1943 * t1580;
    let t8783 = t72 * t2075;
    let t8787 = t1953 + t2081 + t8759 - t8761 / 18.0_f64 - t8764 / 6.0_f64 - t1526 * t3088 * t8767 / 9.0_f64 - t1526 * t1527 * t1970 / 6.0_f64 + t1526 * t1527 * t8775 / 6.0_f64 - t1526 * t1527 * t8779 / 12.0_f64 - t342 * t343 * t8783 / 4.0_f64;
    t8787
}
