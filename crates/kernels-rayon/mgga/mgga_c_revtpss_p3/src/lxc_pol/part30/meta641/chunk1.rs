//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2229/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2229(t15904: f64, t26865: f64, t13127: f64, t17400: f64, t26866: f64, t1802: f64, t3089: f64, t3717: f64, t13148: f64, t17558: f64, t17625: f64, t17713: f64, t17756: f64, t17786: f64, t29100: f64, t3723: f64, t7624: f64, t97136: f64, t97141: f64, t97154: f64, t97161: f64, t97179: f64, sigma2: f64) -> (f64, f64, f64) {
    let t104695 = t26865 * t15904;
    let t104696 = t13127 * t104695;
    let t104703 = t17400 * t26866;
    let t104706 = sigma2 * t1802;
    let t104707 = t104706 * t3089;
    let t104708 = t3717 * t104707;
    let t104715 = t13148 * t104695;
    let t104718 = -0.42874018118069736972e-3_f64 * t29100 * t17786 + 0.42874018118069736972e-3_f64 * t104696 * t17756 + 0.3811023832717309953e-3_f64 * t97136 + 0.47637797908966374413e-3_f64 * t7624 * t17558 + 0.1270341277572436651e-3_f64 * t97141 - 0.85748036236139473944e-3_f64 * t104703 * t3723 + 0.45732285992607719436e-2_f64 * t104708 * t3723 + 0.85748036236139473944e-3_f64 * t97179 * t17625 - 0.3811023832717309953e-3_f64 * t97154 + 0.31758531939310916275e-3_f64 * t97161 + 0.25724410870841842183e-2_f64 * t104715 * t17713;
    (t104695, t104707, t104718)
}
