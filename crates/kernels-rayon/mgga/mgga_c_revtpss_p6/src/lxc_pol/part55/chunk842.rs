//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 842/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk842(t1946: f64, t2684: f64, t7043: f64, t820: f64, t843: f64, t857: f64, t240: f64, t7036: f64, t2664: f64, t2661: f64, t2670: f64, t7033: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25219 = t1946 * t2684;
    let t25220 = 0.11337795902333997111e-1_f64 * t25219;
    let t25222 = t820 * t7043 * t843;
    let t25223 = t25222 * t857;
    let t25224 = 0.16006300097412701803e-1_f64 * t25223;
    let t25227 = t7036 * t240;
    let t25228 = t25227 * t2664;
    let t25229 = t2661 * t25228;
    let t25230 = 0.28582678745379824648e-4_f64 * t25229;
    let t25231 = t7033 * t2670;
    (t25219, t25220, t25222, t25223, t25224, t25227, t25229, t25230, t25231)
}
