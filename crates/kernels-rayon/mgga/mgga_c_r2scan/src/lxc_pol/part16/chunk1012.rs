//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1012/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1012(t3332: f64, t9296: f64, t6535: f64, t3610: f64, t7601: f64, t9292: f64, t2147: f64, t1055: f64, t3179: f64, t9380: f64, t6165: f64, t3308: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12533 = t3332 * t9296;
    let t12534 = t6535 * t12533;
    let t12536 = t7601 * t3610;
    let t12538 = t3332 * t9292;
    let t12539 = t2147 * t12538;
    let t12541 = t3179 * t1055;
    let t12543 = t3332 * t9380;
    let t12544 = t6165 * t12543;
    let t12547 = t3308 * t9380;
    (t12533, t12534, t12536, t12538, t12539, t12541, t12543, t12544, t12547)
}
