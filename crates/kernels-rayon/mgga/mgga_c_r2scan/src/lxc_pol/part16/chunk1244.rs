//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1244/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1244(t3446: f64, t3453: f64, t9056: f64, t12435: f64, t3308: f64, t3429: f64, t1102: f64, t3314: f64, t37387: f64, t39261: f64, t42431: f64, t42435: f64, t42437: f64, t42441: f64, t42443: f64, t42447: f64, t42450: f64, t42452: f64, t42457: f64, t42460: f64) -> f64 {
    let t43826 = t3446 * t3453 * t9056;
    let t43829 = t3429 * t3308 * t12435;
    let t43832 = t1102 * t3314 * t12435;
    let t43834 = -t42431 + t42435 - 0.36021158228745895953e-3_f64 * t43826 - 0.15243824895787514157e-3_f64 * t43829 - 0.40650199722100037752e-3_f64 * t43832 + t42437 - t42441 - t42443 - t37387 + t42447 - t42450 - t39261 + t42452 + t42457 - t42460;
    t43834
}
