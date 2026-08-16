//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1644/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1644(t4003: f64, t6843: f64, t2723: f64, t6016: f64, t197: f64, t531: f64, t2013: f64) -> (f64, f64, f64, f64) {
    let t23037 = t4003 * t6843;
    let t23160 = t2723 * t6016;
    let t25081 = t197 * t531;
    let t25082 = t2013 * t25081;
    (t23037, t23160, t25081, t25082)
}
