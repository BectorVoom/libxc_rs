//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1058/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1058(t2852: f64, t5825: f64, t11354: f64, t6113: f64, t11358: f64, t6132: f64, t698: f64, t6135: f64, t6138: f64, t300: f64, t6184: f64, t6104: f64, t914: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18936 = t2852 * t5825;
    let t18979 = t11354 * t6113;
    let t18987 = t11358 * t6113;
    let t19002 = t698 * t6132;
    let t19004 = t698 * t6135;
    let t19009 = t698 * t6138;
    let t19049 = t300 * t6184;
    let t19056 = t6104 * t914;
    (t18936, t18979, t18987, t19002, t19004, t19009, t19049, t19056)
}
