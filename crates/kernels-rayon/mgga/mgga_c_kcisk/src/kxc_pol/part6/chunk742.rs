//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 742/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk742(t140: f64, t15189: f64, t191: f64, t918: f64, t974: f64, t139: f64, t969: f64, t1003: f64, t2933: f64, t932: f64, t132: f64, t2934: f64) -> (f64, f64, f64, f64, f64) {
    let t15191 = t140 * t15189 * t191;
    let t15193 = t918 * t974;
    let t15195 = t140 * t15193 * t191;
    let t15197 = t139 * t969;
    let t15198 = t15197 * t1003;
    let t15200 = t2933 * t932;
    let t15202 = 1.0_f64 / t2934 / t132;
    (t15191, t15195, t15198, t15200, t15202)
}
