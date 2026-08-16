//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1233/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1233(t13396: f64, t226: f64, t4265: f64, t814: f64, t225: f64, t4149: f64, t4351: f64, t892: f64, t1543: f64, t2841: f64, t4389: f64, t699: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13397 = t226 * t13396;
    let t13433 = t814 * t4265;
    let t13463 = t4149 * t225;
    let t13515 = t4351 * t892;
    let t13520 = t1543 * t2841;
    let t13550 = t699 * t4389;
    (t13397, t13433, t13463, t13515, t13520, t13550)
}
