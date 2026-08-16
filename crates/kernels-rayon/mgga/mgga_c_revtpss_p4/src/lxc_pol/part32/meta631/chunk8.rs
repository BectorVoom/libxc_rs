//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2051/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2051(t110058: f64, t110102: f64, t111089: f64, t111130: f64, t111174: f64, t111214: f64, t111260: f64, t111301: f64, t116: f64, t30570: f64, t109368: f64, t117: f64, t1459: f64, t1916: f64, t2113: f64, t2115: f64, t22544: f64, t22559: f64, t22565: f64, t28975: f64, t28981: f64, t28987: f64, t28990: f64, t30654: f64, t30657: f64, t34359: f64, t4292: f64, t572: f64, t573: f64, t5795: f64, t5802: f64, t670: f64, t6941: f64, t6945: f64, t7547: f64, t7554: f64, t8118: f64, t8124: f64, param_d: f64) -> (f64, f64) {
    let t111304 = t110058 + t110102 + t111089 + t111130 + t111174 + t111214 + t111260 + t111301;
    let t111320 = t116 * t30570;
    let t111345 = 3.0_f64 * t109368 * t117 * t572 + t111304 * t573 * param_d + 6.0_f64 * t111320 * t572 * t670 + 12.0_f64 * t34359 * t4292 * t572 + 12.0_f64 * t1459 * t30654 + 6.0_f64 * t1459 * t30657 + 12.0_f64 * t1916 * t28975 + 12.0_f64 * t1916 * t28981 + 12.0_f64 * t1916 * t28987 + 6.0_f64 * t1916 * t28990 + 12.0_f64 * t2113 * t22559 + 6.0_f64 * t2113 * t22565 + 3.0_f64 * t2115 * t22544 + 12.0_f64 * t5795 * t8124 + 12.0_f64 * t5802 * t8118 + 6.0_f64 * t6941 * t7554 + 6.0_f64 * t6945 * t7547;
    (t111304, t111345)
}
