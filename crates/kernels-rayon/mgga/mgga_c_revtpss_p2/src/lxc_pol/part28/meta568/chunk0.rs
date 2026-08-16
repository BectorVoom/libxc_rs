//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2028/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2028(t2466: f64, t93329: f64, t25375: f64, t7015: f64, t9292: f64, t25411: f64, t93183: f64, t25431: f64, t93123: f64, t25387: f64, t93285: f64, t7063: f64, t860: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t93330 = t93329 * t2466;
    let t93331 = t25375 * t93330;
    let t93334 = 0.17073386770573548589e-1_f64 * t9292 * t7015;
    let t93335 = t25411 * t93183;
    let t93337 = t25431 * t93123;
    let t93339 = t25387 * t93285;
    let t93341 = t7063 * t860;
    (t93330, t93331, t93334, t93335, t93337, t93339, t93341)
}
