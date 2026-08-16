//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1128/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1128(t4265: f64, t4275: f64, t242: f64, t3060: f64, t5254: f64, t3080: f64, t5243: f64, t1111: f64, t1128: f64, t15235: f64, t15262: f64, t4219: f64) -> (f64, f64, f64, f64, f64) {
    let t15519 = t4265 * t4275;
    let t15522 = t242 * t3060 * t5254;
    let t15523 = t3080 * t15522;
    let t15526 = t242 * t3060 * t5243;
    let t15527 = t1111 * t15526;
    let t15533 = t242 * t1128 * t15235;
    let t15536 = t4219 * t15262;
    (t15519, t15523, t15527, t15533, t15536)
}
