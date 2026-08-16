//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1228/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1228(t212: f64, t25286: f64, t689: f64, t780: f64, t10073: f64, t1958: f64, t25390: f64, t886: f64, t10665: f64, t1949: f64, t1955: f64, t25308: f64, t2769: f64) -> (f64, f64, f64, f64) {
    let t92901 = t689 * t212 * t25286 * t780;
    let t92905 = t10073 * t25390 * t1958 * t886;
    let t92907 = t1949 * t10665;
    let t92917 = t1955 * t25308 * t2769;
    (t92901, t92905, t92907, t92917)
}
