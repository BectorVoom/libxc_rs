//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 590/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk590(t15163: f64, t7204: f64, t15166: f64, t7192: f64, t15169: f64, t8620: f64, t3069: f64, t8659: f64, t3077: f64, t8365: f64, t128: f64, t589: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15187 = t7204 * t15163;
    let t15189 = t7192 * t15166;
    let t15191 = t8620 * t15169;
    let t15197 = t8659 * t3069;
    let t15199 = t8365 * t3077;
    let t15203 = t128 * t589;
    (t15187, t15189, t15191, t15197, t15199, t15203)
}
