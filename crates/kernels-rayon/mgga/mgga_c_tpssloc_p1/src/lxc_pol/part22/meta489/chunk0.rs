//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1910/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1910(t21283: f64, t21298: f64, t913: f64, t893: f64, t10704: f64, t21252: f64, t10702: f64, t1568: f64, t17547: f64, t1581: f64, t5790: f64, t1580: f64, t17492: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21299 = t21283 + t21298;
    let t21300 = t21299 * t913;
    let t21302 = 1.0_f64 * t893 * t21300;
    let t21303 = t21252 * t10704;
    let t21305 = 0.51726012919273400301e3_f64 * t10702 * t21303;
    let t21306 = t17547 * t1568;
    let t21309 = t1581 * t5790;
    let t21312 = t17492 * t1580;
    (t21299, t21300, t21302, t21303, t21305, t21306, t21309, t21312)
}
