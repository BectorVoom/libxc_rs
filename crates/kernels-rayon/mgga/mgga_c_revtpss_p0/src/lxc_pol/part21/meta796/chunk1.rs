//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2879/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2879(t11384: f64, t1596: f64, t11388: f64, t52201: f64, t52204: f64, t52207: f64, t52209: f64, t52211: f64, t52213: f64, t52216: f64, t52218: f64, t52221: f64, t52223: f64) -> (f64, f64) {
    let t52224 = t1596 * t11384;
    let t52226 = 0.51726012919273400301e3_f64 * t52224 * t11388;
    let t52227 = -t52201 - t52204 - t52207 - t52209 + t52211 - t52213 + t52216 + t52218 + t52221 + t52223 + t52226;
    (t52226, t52227)
}
