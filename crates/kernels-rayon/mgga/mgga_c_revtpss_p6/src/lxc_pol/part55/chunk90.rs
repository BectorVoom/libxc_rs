//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 90/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk90(t252: f64, t257: f64, t213: f64, t149: f64, t191: f64, t194: f64, t198: f64, t207: f64) -> (f64, f64, f64) {
    let t258 = t252 * t257;
    let t261 = 1.0_f64 + 0.65854491829355115987e0_f64 * t213 * t258;
    let t262 = f64::ln(t261);
    let t265 = t198 * t207 * t262 - t149 + t191 + t194;
    (t261, t262, t265)
}
