//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1147/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1147(t5229: f64, t943: f64, t1108: f64, t938: f64, t15286: f64, t4223: f64, t15281: f64, t4219: f64, t12399: f64, t14906: f64, t3931: f64, t1098: f64, t1116: f64, t1125: f64, t12439: f64, t12443: f64, t4212: f64, t4220: f64, t4224: f64, t9658: f64, t9669: f64, t9701: f64) -> f64 {
    let t15805 = t5229 * t943;
    let t15807 = t938 * t1108 * t15805;
    let t15814 = t4223 * t15286;
    let t15819 = t4219 * t15281;
    let t15822 = t12399 * t14906;
    let t15823 = t3931 * t15822;
    let t15826 = 19.0_f64 / 1728.0_f64 * t15807 * t1116 + t9658 / 1296.0_f64 + t9669 / 20736.0_f64 + t4212 * t4224 / 27.0_f64 - t1098 * t15814 / 144.0_f64 - 2.0_f64 / 81.0_f64 * t4212 * t4220 + t1098 * t15819 / 216.0_f64 + t12439 + t12443 + t9701 - t1125 * t15823 / 768.0_f64;
    t15826
}
