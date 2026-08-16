//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3315/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3315(t85312: f64, t85343: f64, t85373: f64, t86891: f64, t1921: f64, t6936: f64, t1913: f64, t6951: f64, t25072: f64, t571: f64, t116: f64, t117: f64, t1459: f64, t1461: f64, t18207: f64, t1916: f64, t1918: f64, t21881: f64, t22544: f64, t22556: f64, t22559: f64, t22565: f64, t22568: f64, t22633: f64, t25055: f64, t25063: f64, t25066: f64, t25069: f64, t4292: f64, t572: f64, t573: f64, t5795: f64, t5801: f64, t5802: f64, t5805: f64, t5883: f64, t5920: f64, t60595: f64, t670: f64, t6941: f64, t6945: f64, t6948: f64, t75931: f64, param_d: f64) -> (f64, f64, f64, f64, f64) {
    let t86893 = t85312 + t85343 + t85373 + t86891;
    let t86897 = t6936 * t1921;
    let t86903 = t1913 * t6951;
    let t86909 = t571 * t25072;
    let t86958 = param_d * t86893 * t573 + 3.0_f64 * t25055 * t1461 + 9.0_f64 * t22544 * t1918 + 18.0_f64 * t6941 * t5802 + 9.0_f64 * t6941 * t5805 + 18.0_f64 * t5795 * t6945 + 18.0_f64 * t1916 * t22556 + 36.0_f64 * t1916 * t22559 + 9.0_f64 * t5795 * t6948 + 18.0_f64 * t1916 * t22565 + 9.0_f64 * t1916 * t22568 + 6.0_f64 * t1459 * t25063 + 18.0_f64 * t572 * t5883 * t4292 + 18.0_f64 * t1459 * t25066 + 18.0_f64 * t572 * t60595 * t5920 + 18.0_f64 * t572 * t18207 * t5920 + 18.0_f64 * t572 * t5801 * t21881 + 3.0_f64 * t1459 * t25069 + 6.0_f64 * t572 * t116 * t22633 * t670 + 3.0_f64 * t572 * t117 * t75931;
    (t86893, t86897, t86903, t86909, t86958)
}
