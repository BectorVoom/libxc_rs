//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1859/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1859(t25876: f64, t26304: f64, t25894: f64, t2435: f64, t26355: f64, t2097: f64, t22: f64, t25937: f64, t94696: f64, t10115: f64, t2099: f64, t26072: f64, t26292: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t96186 = t25876 * t26304;
    let t96187 = t25894 * t96186;
    let t96197 = t2435 * t26355;
    let t96204 = t25937 * t2097 * t22;
    let t96206 = 0.43639970290213137151e-3_f64 * t94696 * t96204;
    let t96210 = 0.11044544084478153697e-3_f64 * t10115 * t2099;
    let t96211 = t26072 * t26292;
    (t96186, t96187, t96197, t96204, t96206, t96210, t96211)
}
