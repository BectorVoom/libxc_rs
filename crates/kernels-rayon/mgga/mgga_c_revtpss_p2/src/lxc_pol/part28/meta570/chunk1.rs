//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2031/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2031(t25310: f64, t25331: f64, t25412: f64, t93329: f64, t25411: f64, t25431: f64, t2435: f64, t25339: f64, t11064: f64, t7086: f64, t25624: f64, t3056: f64, t7143: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93384 = t25310 * t25331;
    let t93386 = t93329 * t25412;
    let t93387 = t25411 * t93386;
    let t93389 = t25431 * t93386;
    let t93391 = t2435 * t25339;
    let t93404 = t7086 * t11064;
    let t93429 = t25624 * t3056 * t7143;
    (t93384, t93387, t93389, t93391, t93404, t93429)
}
