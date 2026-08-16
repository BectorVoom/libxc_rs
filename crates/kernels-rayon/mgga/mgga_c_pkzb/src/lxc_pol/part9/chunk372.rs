//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 372/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk372(t1410: f64, t1413: f64, t1414: f64, t1444: f64, t1449: f64, t1450: f64, t1466: f64, t42: f64, t430: f64, t453: f64, t55: f64, t58: f64, t63: f64) -> (f64, f64) {
    let t1469 = 0.165625e-1_f64 * t1410 * t42 - 0.6625e-1_f64 * t1413 * t1414 + 0.165625e-1_f64 * t430 * t1444 + 0.496875e-1_f64 * t1449 * t1450 - 0.165625e-1_f64 * t453 * t1466;
    let t1475 = 1.0_f64 / t58 / t55 * t63;
    (t1469, t1475)
}
