//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 754/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk754(t14103: f64, t14152: f64, t14269: f64, t15020: f64, t14372: f64, t15262: f64, t16156: f64, t15254: f64, t14229: f64, t8576: f64, t14255: f64, t3148: f64, t3151: f64, t38471: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t73660 = 0.33133663046638785508e-1_f64 * t14103;
    let t73666 = 0.13010691197123848593e-4_f64 * t14152;
    let t73678 = 0.34695176525663596248e-4_f64 * t14269;
    let t73679 = 2.0_f64 * t15020;
    let t73680 = 0.8175676176687304687e-5_f64 * t14372;
    let t73688 = t16156 * t15262;
    let t73690 = t16156 * t15254;
    let t73691 = 0.19863479950205658386e-4_f64 * t73690;
    let t73692 = t8576 * t14229;
    let t73693 = t73692 * t14255;
    let t73696 = t38471 * t3148 * t3151;
    (t73660, t73666, t73678, t73679, t73680, t73688, t73691, t73692, t73693, t73696)
}
