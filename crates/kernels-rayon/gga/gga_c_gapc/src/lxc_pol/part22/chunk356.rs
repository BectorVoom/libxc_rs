//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 356/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk356(t1576: f64, t1603: f64, t1184: f64, t1206: f64, t1214: f64, t1222: f64, t1229: f64, t1240: f64, t1574: f64, t1578: f64, t1581: f64, t1584: f64, t1588: f64, t1592: f64, t1596: f64, t1600: f64, t434: f64, t469: f64) -> f64 {
    let t1604 = t1576 * t1603;
    let t1607 = -0.1013812832824605378e-3_f64 * t1574 * t1578 - 0.6951859425083008306e-4_f64 * t1581 * t469 - 0.20855578275249024918e-2_f64 * t434 * t1584 - 0.10427789137624512459e-2_f64 * t434 * t1588 + t1222 - t1184 + t1229 - t1240 - 0.12360406057797588768e-3_f64 * t1592 * t1596 + t1206 + t1214 - 0.1013812832824605378e-3_f64 * t1600 * t1604;
    t1607
}
