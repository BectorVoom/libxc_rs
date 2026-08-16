//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1023/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1023(t146: f64, t19875: f64, t252: f64, t545: f64, t6394: f64, t1415: f64, t57: f64, t2252: f64, t6212: f64, t1234: f64, t19790: f64, t560: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19877 = t146 * t19875 * t252;
    let t19883 = t545 * t6394;
    let t20094 = t1415 * t57;
    let t20102 = t6212 * t2252;
    let t20132 = t6212 * t1234;
    let t20146 = t19790 * t560;
    (t19877, t19883, t20094, t20102, t20132, t20146)
}
