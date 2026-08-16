//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3658/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3658(t1196: f64, t12552: f64, t16811: f64, t6534: f64, t16643: f64, t5192: f64, t45232: f64, t68253: f64, t68255: f64, t68257: f64, t68262: f64, t68267: f64, t68271: f64, t68275: f64, t68277: f64, t68282: f64, t68287: f64, t68292: f64) -> (f64, f64, f64) {
    let t69115 = 0.10254018858216406658e4_f64 * t1196 * t12552 * t6534 * t16811;
    let t69117 = 0.2077903092681775651e3_f64 * t5192 * t16643;
    let t69139 = 0.68493333333333333332e-1_f64 * t68253 + 0.76103703703703703702e-2_f64 * t68255 - 0.50735802469135802467e-2_f64 * t68257 + t45232 - 0.12683950617283950617e-1_f64 * t68262 + 0.19025925925925925925e-1_f64 * t68267 + 0.41096e0_f64 * t68271 + 0.68493333333333333332e-1_f64 * t68275 - 0.2283111111111111111e-1_f64 * t68277 - 0.2283111111111111111e-1_f64 * t68282 - 0.11415555555555555555e-1_f64 * t68287 - 0.68493333333333333331e-1_f64 * t68292;
    (t69115, t69117, t69139)
}
