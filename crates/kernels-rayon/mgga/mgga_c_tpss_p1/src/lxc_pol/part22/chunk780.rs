//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 780/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk780(t3426: f64, t4283: f64, t3931: f64, t1128: f64, t4056: f64, t242: f64, t1116: f64, t1125: f64, t1130: f64, t3063: f64, t3067: f64, t3080: f64, t3089: f64, t3093: f64, t4253: f64, t4258: f64, t4261: f64, t4265: f64, t4271: f64, t4276: f64, t4280: f64) -> (f64, f64) {
    let t4284 = t4283 * t3426;
    let t4285 = t3931 * t4284;
    let t4288 = t1128 * t4056;
    let t4289 = t242 * t4288;
    let t4292 = -t3080 * t4253 / 3072.0_f64 - t4258 * t1116 / 576.0_f64 - t4261 / 864.0_f64 + t4265 * t1130 / 864.0_f64 + t3063 / 4608.0_f64 - t3089 - t3093 / 6912.0_f64 - t3067 * t4271 / 4608.0_f64 - t4276 / 6912.0_f64 + 5.0_f64 / 13824.0_f64 * t1125 * t4280 - t1125 * t4285 / 2304.0_f64 - t1125 * t4289 / 4608.0_f64;
    (t4284, t4292)
}
