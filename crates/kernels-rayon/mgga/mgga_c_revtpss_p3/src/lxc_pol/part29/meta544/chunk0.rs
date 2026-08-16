//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1880/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1880(t25900: f64, t96191: f64, t25904: f64, t26231: f64, t94802: f64, t2435: f64, t26355: f64, t2097: f64, t22: f64, t25937: f64, t94696: f64, t10115: f64, t2099: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t96192 = t96191 * t25900;
    let t96193 = t25904 * t96192;
    let t96195 = t94802 * t26231;
    let t96197 = t2435 * t26355;
    let t96204 = t25937 * t2097 * t22;
    let t96206 = 0.43639970290213137151e-3_f64 * t94696 * t96204;
    let t96210 = 0.11044544084478153697e-3_f64 * t10115 * t2099;
    (t96192, t96193, t96195, t96197, t96204, t96206, t96210)
}
