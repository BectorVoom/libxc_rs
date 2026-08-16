//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1068/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1068(t12803: f64, t5352: f64, t3720: f64, t1209: f64, t3781: f64, t5330: f64, t3153: f64, t3601: f64, t12269: f64, t247: f64, t3618: f64, t12277: f64, t1264: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12804 = t12803 * t5352;
    let t12805 = t3720 * t12804;
    let t12808 = t1209 * t3781;
    let t12809 = t12808 * t5330;
    let t12810 = t3601 * t3153;
    let t12811 = t12810 * t5352;
    let t12812 = t3720 * t12811;
    let t12816 = t247 * t3618 * t12269;
    let t12822 = t247 * t1264 * t12277;
    (t12805, t12809, t12810, t12812, t12816, t12822)
}
