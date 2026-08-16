//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 607/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk607(t695: f64, t719: f64, t1060: f64, t1894: f64, t5184: f64, t5182: f64, t716: f64, t654: f64, sigma2: f64) -> (f64, f64, f64, f64) {
    let t5185 = t719 * t695;
    let t5186 = t1060 * t1894;
    let t5187 = t5185 * t5186;
    let t5188 = t5184 * t5187;
    let t5189 = t5182 * t5188;
    let t5191 = t716 * sigma2;
    let t5192 = t5191 * t654;
    (t5185, t5188, t5189, t5192)
}
