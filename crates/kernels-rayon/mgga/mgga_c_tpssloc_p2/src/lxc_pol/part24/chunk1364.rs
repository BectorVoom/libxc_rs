//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1364/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1364(t225: f64, t23408: f64, t1921: f64, t6733: f64, t3034: f64, t336: f64, t131: f64, t350: f64, t38: f64, t10469: f64, t344: f64, t10474: f64) -> (f64, f64, f64, f64, f64) {
    let t82499 = t23408 * t225;
    let t82502 = t6733 * t1921;
    let t82510 = 1.0_f64 / t3034 / t336;
    let t82513 = t38 * t82510 * t131 * t350;
    let t82514 = t344 * t10469;
    let t82515 = t82514 * t10474;
    (t82499, t82502, t82513, t82514, t82515)
}
