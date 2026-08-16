//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1149/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1149(t15842: f64, t3931: f64, t461: f64, t5248: f64, t4232: f64, t1015: f64, t5254: f64, t3068: f64, t1125: f64, t12435: f64, t12446: f64, t12448: f64, t12472: f64, t15828: f64, t15832: f64, t15835: f64, t15839: f64, t3052: f64, t3080: f64, t4253: f64, t4271: f64, t9573: f64, t9626: f64) -> (f64, f64) {
    let t15843 = t3931 * t15842;
    let t15846 = t461 * t5248;
    let t15847 = t15846 * t4232;
    let t15848 = t3931 * t15847;
    let t15854 = t5254 * t1015;
    let t15855 = t3068 * t15854;
    let t15860 = -t1125 * t15828 / 1152.0_f64 - t12446 / 6912.0_f64 + t15832 / 162.0_f64 - t15835 / 864.0_f64 + t3052 * t15839 / 768.0_f64 - t3080 * t15843 / 1536.0_f64 - t9626 * t15848 / 512.0_f64 + t12448 / 1296.0_f64 + t12435 * t4253 / 288.0_f64 + t9573 * t15855 / 4608.0_f64 + t12472 * t4271 / 432.0_f64;
    (t15846, t15860)
}
