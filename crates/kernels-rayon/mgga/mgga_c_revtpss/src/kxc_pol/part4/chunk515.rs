//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 515/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk515(t21: f64, t25: f64, t2219: f64, t2221: f64, t2223: f64, t2226: f64, t2228: f64, t2230: f64, t2233: f64, t2235: f64, t599: f64, t602: f64) -> (f64, f64, f64, f64, f64) {
    let t2236 = t21 * t21;
    let t2237 = 1.0_f64 / t2236;
    let t2239 = 42.0_f64 * t25 * t2237;
    let t2240 = t2219 - t2221 + t2223 + t2226 - t2228 + t2230 + t2233 - t2235 + t2239;
    let t2242 = t599 * t602;
    (t2236, t2237, t2239, t2240, t2242)
}
