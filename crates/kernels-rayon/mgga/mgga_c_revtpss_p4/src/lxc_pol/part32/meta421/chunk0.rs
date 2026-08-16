//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1473/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1473(t15935: f64, t19661: f64, t1042: f64, t19666: f64, t4801: f64, t1592: f64, t16138: f64, t19399: f64, t247: f64, t3116: f64, t18942: f64, t4915: f64) -> (f64, f64, f64, f64, f64) {
    let t19929 = t15935 * t19661;
    let t19930 = t1042 * t19929;
    let t19933 = t4801 * t19666;
    let t19934 = t1042 * t19933;
    let t19939 = t16138 * t1592;
    let t19940 = t1042 * t19939;
    let t19944 = t247 * t3116 * t19399;
    let t19947 = t4915 * t18942;
    (t19930, t19934, t19940, t19944, t19947)
}
