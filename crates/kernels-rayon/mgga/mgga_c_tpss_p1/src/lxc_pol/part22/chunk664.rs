//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 664/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk664(t3096: f64, t66: f64, t2841: f64, t242: f64, t1128: f64, t2846: f64, t2850: f64, t1098: f64, t1111: f64, t1125: f64, t3027: f64, t3029: f64, t3035: f64, t3040: f64, t3044: f64, t3052: f64, t3057: f64, t3063: f64, t3067: f64, t3070: f64, t3076: f64, t3080: f64, t3083: f64, t3089: f64, t3093: f64) -> (f64, f64, f64, f64, f64) {
    let t3097 = t66 * t3096;
    let t3098 = t3097 * t2841;
    let t3099 = t242 * t3098;
    let t3102 = t1128 * t2846;
    let t3103 = t242 * t3102;
    let t3106 = t1128 * t2850;
    let t3107 = t242 * t3106;
    let t3110 = -t3027 - t3029 / 432.0_f64 + t1098 * t3035 / 216.0_f64 - t1098 * t3040 / 144.0_f64 - t1098 * t3044 / 288.0_f64 + t3052 * t3057 / 1536.0_f64 + t3063 / 2304.0_f64 - t3067 * t3070 / 2304.0_f64 + t1111 * t3076 / 3072.0_f64 - t3080 * t3083 / 3072.0_f64 - t3089 - t3093 / 3456.0_f64 + 5.0_f64 / 13824.0_f64 * t1125 * t3099 - t1125 * t3103 / 2304.0_f64 - t1125 * t3107 / 4608.0_f64;
    (t3097, t3099, t3103, t3107, t3110)
}
