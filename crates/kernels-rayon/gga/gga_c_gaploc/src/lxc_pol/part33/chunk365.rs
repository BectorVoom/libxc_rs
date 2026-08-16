//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 365/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk365(t1603: f64, t529: f64, t1324: f64, t531: f64, t1265: f64, t600: f64, t568: f64, t569: f64, t417: f64, t423: f64, t566: f64, t1305: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1604 = t1603 * t529;
    let t1605 = t531 * t1324;
    let t1608 = t600 * t1265;
    let t1609 = t568 * t1608;
    let t1612 = t569 * t1265;
    let t1613 = t568 * t1612;
    let t1616 = t417 * t423;
    let t1617 = t1616 * t566;
    let t1620 = t569 * t1305;
    (t1604, t1605, t1609, t1613, t1616, t1617, t1620)
}
