//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1739/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1739<F: Float>(t21660: F, t22531: F, t3: F, t5883: F, t670: F, t4292: F, t5801: F, t116: F, t5920: F, t117: F, t21881: F, t1459: F, t1461: F, t1916: F, t1918: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F, t6941: F, t6945: F, t6948: F, param_d: F) -> (F, F, F, F, F, F, F) {
    let t22532 = t21660 + t22531;
    let t22533 = t3 * t22532;
    let t22544 = param_d * t22532;
    let t22556 = t670 * t5883;
    let t22559 = t5801 * t4292;
    let t22564 = t116 * t5920;
    let t22565 = t22564 * t670;
    let t22568 = t117 * t21881;
    let t22571 = F::cast_from(6.0_f64) * t1459 * t6945 + F::cast_from(3.0_f64) * t1459 * t6948 + F::cast_from(3.0_f64) * t1461 * t6941 + F::cast_from(12.0_f64) * t1916 * t5802 + F::cast_from(6.0_f64) * t1916 * t5805 + F::cast_from(6.0_f64) * t1918 * t5795 + t22544 * t573 + F::cast_from(6.0_f64) * t22556 * t572 + F::cast_from(12.0_f64) * t22559 * t572 + F::cast_from(6.0_f64) * t22565 * t572 + F::cast_from(3.0_f64) * t22568 * t572;
    (t22533, t22544, t22556, t22559, t22565, t22568, t22571)
}
