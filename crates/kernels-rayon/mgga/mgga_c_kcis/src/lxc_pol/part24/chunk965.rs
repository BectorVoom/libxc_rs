//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 965/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk965(t6400: f64, t962: f64, t19094: f64, t971: f64, t1211: f64, t6783: f64, t10874: f64, t10960: f64, t1221: f64, t1225: f64, t1226: f64, t15445: f64, t1835: f64, t18983: f64, t18987: f64, t18993: f64, t18995: f64, t18999: f64, t3582: f64, t5242: f64, t5250: f64, t6814: f64, t6817: f64, t6820: f64) -> f64 {
    let t20381 = t6400 * t962;
    let t20392 = t19094 * t971;
    let t20397 = t6783 * t1211;
    let t20400 = -t18983 - t18987 + 0.58482233974552040708e0_f64 * t20381 * t1226 + 0.11696446794910408142e1_f64 * t15445 * t1835 + 0.11696446794910408142e1_f64 * t5242 * t5250 - 0.11696446794910408142e1_f64 * t10960 * t6814 + 0.58482233974552040708e0_f64 * t3582 * t6817 + 0.58482233974552040708e0_f64 * t1225 * t20392 + t18993 - t18995 - t18999 + 0.17315755899375863299e2_f64 * t10874 * t6820 + 1.0_f64 * t20397 * t1221;
    t20400
}
