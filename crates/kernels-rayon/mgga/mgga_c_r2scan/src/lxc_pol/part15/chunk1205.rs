//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1205/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1205(t3446: f64, t3447: f64, t40453: f64, t874: f64, t122: f64, t3434: f64, t3437: f64, t3579: f64, t38289: f64, t1563: f64, t2867: f64, t10997: f64, t3275: f64) -> (f64, f64, f64, f64) {
    let t40456 = t3446 * t3447 * t40453 * t874;
    let t40457 = 0.30487649791575028314e-3_f64 * t40456;
    let t40460 = t3434 * t3437 * t40453 * t122;
    let t40461 = 0.43368970657079495312e-4_f64 * t40460;
    let t40463 = t3579 * t38289 / 4.0_f64;
    let t40464 = t2867 * t1563;
    let t40467 = 45.0_f64 / 64.0_f64 * t3275 * t10997 * t40464;
    (t40457, t40461, t40463, t40467)
}
