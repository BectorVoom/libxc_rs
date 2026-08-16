//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1032/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1032(t116: f64, t5821: f64, t114: f64, t923: f64, t3052: f64, t927: f64, t123: f64, t6: f64, t120: f64, t20: f64, t3050: f64, t3058: f64, t397: f64) -> (f64, f64, f64, f64) {
    let t15244 = t116 * t5821;
    let t15245 = t114 * t15244;
    let t15250 = t923 * t923;
    let t15251 = 1.0_f64 / t15250;
    let t15253 = t3052 * t927;
    let t15255 = t123 * t6 * t15251 * t15253;
    let t15258 = t120 * t20;
    let t15259 = t114 * t15258;
    let t15260 = t3050 * t927;
    let t15262 = t397 * t15260 * t3058;
    (t15245, t15255, t15259, t15262)
}
