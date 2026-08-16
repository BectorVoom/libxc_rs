//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 855/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk855(t31139: f64, t544: f64, t986: f64, t2386: f64, t10525: f64, t10526: f64, t41965: f64, t3177: f64, t35091: f64, t9272: f64, t204: f64, t41726: f64, t587: f64) -> (f64, f64, f64, f64) {
    let t42219 = t544 * t31139 * t986;
    let t42221 = 0.25025342966295298669e1_f64 * t42219 * t2386;
    let t42224 = 0.21450293971110256001e1_f64 * t10525 * t10526 * t41965;
    let t42226 = t9272 * t35091 * t3177;
    let t42227 = 0.11502877786176224903e1_f64 * t42226;
    let t42230 = 0.18404604457881959845e2_f64 * t587 * t204 * t41726;
    (t42221, t42224, t42227, t42230)
}
