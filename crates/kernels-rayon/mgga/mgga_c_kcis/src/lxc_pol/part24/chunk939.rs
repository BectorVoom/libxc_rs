//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 939/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk939(t14785: f64, t5099: f64, t15085: f64, t19588: f64, t5180: f64, t19614: f64, t3338: f64, t5046: f64, t19789: f64, t5047: f64, t1130: f64, t19655: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19945 = t14785 * t5099;
    let t19947 = t15085 * t19588;
    let t19948 = t5180 * t19947;
    let t19950 = t3338 * t19614;
    let t19951 = t5046 * t19950;
    let t19953 = t5047 * t19789;
    let t19954 = t5046 * t19953;
    let t19956 = t1130 * t19655;
    (t19945, t19947, t19948, t19950, t19951, t19953, t19954, t19956)
}
