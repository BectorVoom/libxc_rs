//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1109/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1109(t2411: f64, t30: f64, t1946: f64, t2684: f64, t7043: f64, t820: f64, t843: f64, t240: f64, t7036: f64, t2670: f64, t7033: f64, t2482: f64, t27: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25207 = t2411 * t30;
    let t25219 = t1946 * t2684;
    let t25220 = 0.11337795902333997111e-1_f64 * t25219;
    let t25222 = t820 * t7043 * t843;
    let t25227 = t7036 * t240;
    let t25231 = t7033 * t2670;
    let t25232 = 0.27104001498285508387e-3_f64 * t25231;
    let t25234 = t2482 * t7043 * t27;
    (t25207, t25220, t25222, t25227, t25232, t25234)
}
