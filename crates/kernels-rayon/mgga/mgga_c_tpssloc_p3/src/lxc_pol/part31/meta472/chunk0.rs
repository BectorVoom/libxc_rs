//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1632/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1632(t1827: f64, t22765: f64, t5234: f64, t6944: f64, t1354: f64, t22756: f64, t5289: f64, t6945: f64, t5310: f64, t6952: f64, t1824: f64, t236: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26231 = t22765 * t1827;
    let t26233 = t5234 * t6944;
    let t26234 = t26233 * t1354;
    let t26236 = t22756 * t1827;
    let t26238 = t6945 * t5289;
    let t26240 = t6952 * t5310;
    let t26243 = t236 * t1824;
    (t26231, t26233, t26234, t26236, t26238, t26240, t26243)
}
