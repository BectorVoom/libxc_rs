//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1032/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1032(t253: f64, t5134: f64, t2568: f64, t3433: f64, t2563: f64, t2719: f64, t6212: f64, t19790: f64, t938: f64, t2526: f64, t910: f64, t146: f64, t5094: f64, t774: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24063 = t5134 * t253;
    let t24521 = t3433 * t2568;
    let t24573 = t3433 * t2563;
    let t24902 = t6212 * t2719;
    let t24906 = t19790 * t938;
    let t24912 = t6212 * t2526;
    let t24916 = t19790 * t910;
    let t25169 = t146 * t5094 * t774;
    (t24063, t24521, t24573, t24902, t24906, t24912, t24916, t25169)
}
