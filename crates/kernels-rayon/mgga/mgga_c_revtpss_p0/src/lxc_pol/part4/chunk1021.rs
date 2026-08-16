//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1021/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1021(t2645: f64, t2723: f64, t10115: f64, t253: f64, t233: f64, t2760: f64, t869: f64, t689: f64, t2777: f64, t2789: f64, t2439: f64, t2435: f64, t2790: f64) -> (f64, f64, f64, f64, f64) {
    let t10943 = t2723 * t2645;
    let t10948 = 0.11044544084478153697e-3_f64 * t10115 * t253;
    let t10959 = t233 * t2760;
    let t10960 = t869 * t10959;
    let t10961 = t689 * t10960;
    let t10963 = t2777 * t2789;
    let t10964 = t2439 * t10963;
    let t10966 = t2435 * t2790;
    (t10943, t10948, t10961, t10964, t10966)
}
