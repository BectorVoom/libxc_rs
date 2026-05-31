//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3315/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3315<F: Float>(t85312: F, t85343: F, t85373: F, t86891: F, t1921: F, t6936: F, t1913: F, t6951: F, t25072: F, t571: F, t116: F, t117: F, t1459: F, t1461: F, t18207: F, t1916: F, t1918: F, t21881: F, t22544: F, t22556: F, t22559: F, t22565: F, t22568: F, t22633: F, t25055: F, t25063: F, t25066: F, t25069: F, t4292: F, t572: F, t573: F, t5795: F, t5801: F, t5802: F, t5805: F, t5883: F, t5920: F, t60595: F, t670: F, t6941: F, t6945: F, t6948: F, t75931: F, param_d: F) -> (F, F, F, F, F) {
    let t86893 = t85312 + t85343 + t85373 + t86891;
    let t86897 = t6936 * t1921;
    let t86903 = t1913 * t6951;
    let t86909 = t571 * t25072;
    let t86958 = param_d * t86893 * t573 + F::cast_from(3.0_f64) * t25055 * t1461 + F::cast_from(9.0_f64) * t22544 * t1918 + F::cast_from(18.0_f64) * t6941 * t5802 + F::cast_from(9.0_f64) * t6941 * t5805 + F::cast_from(18.0_f64) * t5795 * t6945 + F::cast_from(18.0_f64) * t1916 * t22556 + F::cast_from(36.0_f64) * t1916 * t22559 + F::cast_from(9.0_f64) * t5795 * t6948 + F::cast_from(18.0_f64) * t1916 * t22565 + F::cast_from(9.0_f64) * t1916 * t22568 + F::cast_from(6.0_f64) * t1459 * t25063 + F::cast_from(18.0_f64) * t572 * t5883 * t4292 + F::cast_from(18.0_f64) * t1459 * t25066 + F::cast_from(18.0_f64) * t572 * t60595 * t5920 + F::cast_from(18.0_f64) * t572 * t18207 * t5920 + F::cast_from(18.0_f64) * t572 * t5801 * t21881 + F::cast_from(3.0_f64) * t1459 * t25069 + F::cast_from(6.0_f64) * t572 * t116 * t22633 * t670 + F::cast_from(3.0_f64) * t572 * t117 * t75931;
    (t86893, t86897, t86903, t86909, t86958)
}
