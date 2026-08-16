//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2078/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2078(t16524: f64, t23896: f64, t45560: f64, t7769: f64, t16521: f64, t6534: f64, t1873: f64, t55405: f64, t23893: f64, t12524: f64, t26550: f64, t16535: f64, t7467: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t86639 = 27.0_f64 * t16524 * t23896;
    let t86642 = 27.0_f64 * t45560 * t7769;
    let t86646 = 27.0_f64 * t16521 * t6534;
    let t86651 = 27.0_f64 * t55405 * t1873;
    let t86653 = 54.0_f64 * t16524 * t23893;
    let t86655 = 54.0_f64 * t12524 * t26550;
    let t86660 = 27.0_f64 * t16535 * t7467;
    (t86639, t86642, t86646, t86651, t86653, t86655, t86660)
}
