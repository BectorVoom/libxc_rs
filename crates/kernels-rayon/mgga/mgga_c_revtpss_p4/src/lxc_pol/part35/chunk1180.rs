//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1180/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1180(t23336: f64, t27261: f64, t23323: f64, t25270: f64, t27221: f64, t76613: f64, t23267: f64, t7025: f64, t23263: f64, t92981: f64, t23281: f64, t7045: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t113186 = t27261 * t23336;
    let t113188 = t25270 * t23323;
    let t113214 = t27221 * t76613;
    let t113217 = t7025 * t23267;
    let t113222 = t92981 * t23263;
    let t113226 = t7045 * t23281;
    (t113186, t113188, t113214, t113217, t113222, t113226)
}
