//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1208/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1208(t4147: f64, t7933: f64, t1518: f64, t2126: f64, t11239: f64, t3736: f64, t211: f64, t9644: f64, t138: f64, t785: f64, t9302: f64, t2452: f64, t9720: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33651 = t4147 * t7933;
    let t34446 = t2126 * t1518;
    let t37885 = t11239 * t3736;
    let t39643 = 1.0_f64 / t9644 / t211;
    let t40270 = t138 * t9302 * t785;
    let t40688 = t9720 * t2452;
    (t33651, t34446, t37885, t39643, t40270, t40688)
}
