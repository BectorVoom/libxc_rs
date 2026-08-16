//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 822/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk822(t1805: f64, t818: f64, t5572: f64, t226: f64, t5577: f64, t782: f64, t1708: f64, t228: f64, t5831: f64, t1707: f64, t1809: f64, t253: f64, t5568: f64, t5571: f64, t5832: f64, t5834: f64, t819: f64) -> (f64, f64, f64, f64) {
    let t5837 = t1805 * t818;
    let t5838 = t5572 * t5837;
    let t5843 = t5577 * t1805 * t782 * t226;
    let t5846 = t1708 * t228 * t5831;
    let t5848 = -t1707 * t5846 - t1809 * t5568 + t253 * t5832 + 2.0_f64 * t5571 * t5838 + t5571 * t5843 - t5834 * t819;
    (t5838, t5843, t5846, t5848)
}
