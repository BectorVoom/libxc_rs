//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 610/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk610(t5052: f64, t5210: f64, t752: f64, t1904: f64, t1907: f64, t1957: f64, t1906: f64, t751: f64, t724: f64, t196: f64, t4794: f64, t574: f64, t725: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5211 = t5052 + t5210;
    let t5212 = t5211 * t752;
    let t5213 = t1904 * t1907;
    let t5215 = 2.0_f64 * t5213 * t1957;
    let t5217 = 1.0_f64 / t1906 / t751;
    let t5218 = t724 * t5217;
    let t5219 = t1957 * t1957;
    let t5221 = 2.0_f64 * t5218 * t5219;
    let t5222 = t4794 * t196;
    let t5231 = t725 * t574;
    (t5211, t5212, t5213, t5215, t5217, t5218, t5219, t5221, t5222, t5231)
}
