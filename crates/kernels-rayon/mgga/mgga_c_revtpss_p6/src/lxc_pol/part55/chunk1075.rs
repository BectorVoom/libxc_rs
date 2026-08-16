//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1075/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1075(t644: f64, t8621: f64, t8881: f64, t36: f64, t68: f64, t606: f64, t8442: f64, t84: f64, t640: f64, t624: f64, t8441: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33265 = t8621 * t8881 * t644;
    let t33268 = t68 * t36;
    let t33269 = t33268 * t606;
    let t33270 = t8442 * t33269;
    let t33275 = t84 * t68;
    let t33277 = t8621 * t33275 * t640;
    let t33280 = t8441 * t624;
    let t33281 = t8621 * t33280;
    (t33265, t33268, t33270, t33275, t33277, t33280, t33281)
}
