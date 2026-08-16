//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 370/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk370(t1247: f64, t129: f64, t1240: f64, t3097: f64, t3091: f64, t464: f64, t866: f64, t3095: f64, t3099: f64, t869: f64, t871: f64, t1232: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t3101 = t1247 * t129;
    let t3102 = t3097 * t1240;
    let t3103 = t3102 * pi;
    let t3104 = t3101 * t3103;
    let t3106 = t464 * t3091;
    let t3107 = t3106 * t866;
    let t3108 = t3107 / 256.0_f64;
    let t3109 = t3095 - 9.0_f64 / 8192.0_f64 * t3099 + 3.0_f64 / 8192.0_f64 * t3104 - t3108;
    let t3111 = t869 * t871;
    let t3113 = 1.0_f64 / t1232;
    (t3101, t3102, t3103, t3104, t3106, t3107, t3108, t3109, t3111, t3113)
}
