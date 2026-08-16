//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1152/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1152(t3332: f64, t4415: f64, t4416: f64, t10117: f64, t4473: f64, t10121: f64, t10122: f64, t3256: f64, t339: f64, t790: f64, t4419: f64, t10086: f64, t236: f64) -> (f64, f64, f64, f64, f64) {
    let t12877 = t4415 * t4416 * t3332;
    let t12881 = 7.0_f64 / 576.0_f64 * t10117 * t4473;
    let t12883 = t10121 * t4416 * t10122;
    let t12887 = t339 * t3256 * t790;
    let t12889 = 7.0_f64 / 1152.0_f64 * t12887 * t4419;
    let t12891 = t339 * t10086 * t236;
    (t12877, t12881, t12883, t12889, t12891)
}
