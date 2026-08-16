//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2005/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2005(t10469: f64, t344: f64, t10482: f64, t3032: f64, t131: f64, t23598: f64, t350: f64, t614: f64, t3131: f64, t23383: f64, t6712: f64) -> (f64, f64, f64, f64, f64) {
    let t82514 = t344 * t10469;
    let t82516 = t3032 * t10482;
    let t82534 = t614 * t23598 * t131 * t350;
    let t82542 = t3032 * t3131;
    let t82573 = t6712 * t23383;
    (t82514, t82516, t82534, t82542, t82573)
}
