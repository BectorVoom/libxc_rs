//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 814/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk814(t2365: f64, t28648: f64, t7630: f64, t28431: f64, t787: f64, t9824: f64, t22984: f64, t7584: f64, t9438: f64, t28983: f64, t959: f64, t28846: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41234 = t7630 * t2365 * t28648;
    let t41236 = t787 * t28431;
    let t41237 = t41236 * t9824;
    let t41244 = t7584 * t9438 * t22984;
    let t41281 = t28983 * t959;
    let t41283 = t28846 * t959;
    (t41234, t41236, t41237, t41244, t41281, t41283)
}
