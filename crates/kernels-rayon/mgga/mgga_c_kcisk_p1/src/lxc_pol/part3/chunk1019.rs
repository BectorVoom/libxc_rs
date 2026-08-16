//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1019/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1019(t524: f64, t1576: f64, t4370: f64, t14164: f64, t20: f64, t533: f64, t4502: f64, t1572: f64, t4416: f64, t14866: f64, t1589: f64, t1586: f64, t15006: f64, t15009: f64, t15011: f64, t15014: f64, t15041: f64, t15044: f64, t15047: f64, t15052: f64, t15053: f64, t1583: f64, t535: f64, t541: f64) -> f64 {
    let t536 = 0.0_f64 < t524;
    let t15056 = t4370 * t1576;
    let t15058 = t14164 * t20;
    let t15059 = t533 * t15058;
    let t15062 = t4502 * t1576;
    let t15064 = t1572 * t4416;
    let t15067 = piecewise3(t536, t14866, -t14866);
    let t15068 = t1589 * t15067;
    let t15069 = t1586 * t15068;
    let t15072 = -0.59969295720591057378e-2_f64 * t15006 - 0.17990788716177317214e-1_f64 * t15009 + 0.26386490117060065246e0_f64 * t15011 * t1583 - 0.47975436576472845903e-1_f64 * t15014 + 0.2698618307426597582e-1_f64 * t15041 * t541 + 0.7915947035118019574e0_f64 * t15044 * t541 + 0.26386490117060065246e0_f64 * t15047 + t15052 - 0.21588946459412780656e0_f64 * t15053 * t541 + 0.2698618307426597582e-1_f64 * t15056 - 0.12313695387961363781e1_f64 * t15059 * t541 - 0.14392630972941853771e0_f64 * t15062 + 0.47975436576472845902e-1_f64 * t15064 - 0.2698618307426597582e-1_f64 * t535 * t15069;
    t15072
}
