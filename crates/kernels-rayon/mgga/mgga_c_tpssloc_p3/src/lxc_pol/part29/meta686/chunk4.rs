//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2354/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2354(t111: f64, t8110: f64, t12813: f64, t16541: f64, t2319: f64, t24972: f64, t5376: f64, t7423: f64, t85416: f64, t86631: f64, t86633: f64, t86635: f64, t86637: f64, t86639: f64, t86642: f64, t86646: f64, t86651: f64, t86653: f64, t86655: f64, t86660: f64, t86668: f64, t91799: f64, t91802: f64) -> f64 {
    let t96334 = t8110 * t111;
    let t96337 = 27.0_f64 * t24972 * t16541 + t86631 + 0.135e2_f64 * t7423 * t12813 + t86633 + t86635 + t86637 + t86639 + t86642 + t86646 + t86651 + t86653 + t86655 + t86660 + t86668 + 54.0_f64 * t85416 * t5376 + 27.0_f64 * t96334 * t2319 + t91799 + t91802;
    t96337
}
