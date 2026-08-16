//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 686/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk686(t14123: f64, t3113: f64, t68438: f64, t14045: f64, t14121: f64, t1008: f64, t464: f64, t1966: f64, t220: f64, t14167: f64, t14115: f64, t68447: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t68876 = t3113 * t68438 * t14123;
    let t68884 = t14045 * t14121 * t14123;
    let t68889 = t464 * t1008;
    let t68891 = t1966 * t68889 * t220;
    let t68892 = t68891 * t14167;
    let t68893 = 0.29085809927086856922e-4_f64 * t68892;
    let t68906 = t68447 * t14115;
    (t68876, t68884, t68889, t68891, t68893, t68906)
}
