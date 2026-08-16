//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1213/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1213(t5457: f64, t5468: f64, t10915: f64, t18926: f64, t2938: f64, t43050: f64, t43084: f64, t43250: f64, t631: f64, t69265: f64, t69289: f64, t69291: f64, t82077: f64, t82079: f64, t82088: f64, t82095: f64, t82097: f64, t88252: f64, t898: f64) -> f64 {
    let t91251 = t5457 * t5457;
    let t91264 = t5468 * t5468;
    let t91269 = 12.0_f64 * t82077 - 4.0_f64 / 9.0_f64 * t82079 - 16.0_f64 / 81.0_f64 * t82088 + 10.0_f64 / 9.0_f64 * t69265 - 8.0_f64 / 3.0_f64 * t82095 + 8.0_f64 / 9.0_f64 * t82097 - 20.0_f64 / 9.0_f64 * t69289 - 10.0_f64 * t69291 - 30.0_f64 * t631 * t898 * t43250 * t91251 - t43084 + 36.0_f64 * t631 * t898 * t18926 * t5468 - 8.0_f64 / 9.0_f64 * t631 * t10915 * t43050 * t88252 - 9.0_f64 / 2.0_f64 * t631 * t898 * t2938 * t91264;
    t91269
}
