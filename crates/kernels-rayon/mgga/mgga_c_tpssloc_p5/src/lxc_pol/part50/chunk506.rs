//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 506/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk506(t1229: f64, t154: f64, t636: f64, t2296: f64, t1094: f64, t1098: f64, t1097: f64, t419: f64, t409: f64, t407: f64, t410: f64, t3236: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3240 = t154 * t1229;
    let t3241 = t636 * t636;
    let t3242 = 1.0_f64 / t3241;
    let t3247 = 1.0_f64 / t2296;
    let t3259 = t1094 * t1098;
    let t3262 = t1097 * t419;
    let t3263 = 1.0_f64 / t3262;
    let t3264 = t409 * t3263;
    let t3270 = 1.0_f64 / t410 / t407;
    let t3274 = 4.0_f64 / 9.0_f64 * t3236;
    (t3240, t3242, t3247, t3259, t3264, t3270, t3274)
}
