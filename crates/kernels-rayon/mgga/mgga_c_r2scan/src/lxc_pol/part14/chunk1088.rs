//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1088/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1088(t3281: f64, t6271: f64, t10949: f64, t10992: f64, t2315: f64, t3446: f64, t158: f64, t1783: f64, t3447: f64, t874: f64, t122: f64, t3434: f64, t3437: f64) -> (f64, f64, f64, f64) {
    let t38193 = t3281 * t6271;
    let t38211 = t3446 * t10992 * t10949 * t2315;
    let t38213 = t158 * t1783;
    let t38216 = t3446 * t3447 * t38213 * t874;
    let t38220 = t3434 * t3437 * t38213 * t122;
    (t38193, t38211, t38216, t38220)
}
