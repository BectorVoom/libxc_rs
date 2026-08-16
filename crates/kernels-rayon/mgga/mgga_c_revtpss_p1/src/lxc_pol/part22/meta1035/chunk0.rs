//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3620/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3620(t68284: f64, t68338: f64, t68379: f64, t68466: f64, t68501: f64, t68526: f64, t68564: f64, t68595: f64, t1179: f64, t1188: f64, t1196: f64, t20397: f64, t3531: f64) -> (f64, f64, f64) {
    let t68598 = t68284 + t68338 + t68379 + t68466 + t68501 + t68526 + t68564 + t68595;
    let t68602 = 0.5848223622634646207e0_f64 * t1196 * t1179 * t68598 * t1188;
    let t68604 = 0.69263436422725855036e2_f64 * t3531 * t20397;
    (t68598, t68602, t68604)
}
