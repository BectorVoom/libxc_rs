//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 504/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk504(t475: f64, t9448: f64, t9438: f64, t2487: f64, t203: f64, t539: f64, t107: f64, t6519: f64, t883: f64, t1538: f64, t6583: f64, t2478: f64, t888: f64) -> (f64, f64, f64, f64, f64) {
    let t9449 = t9448 * t475;
    let t9450 = t9438 * t9449;
    let t9451 = t2487 * t9450;
    let t9453 = t539 * t203;
    let t9454 = t107 * t9453;
    let t9537 = t883 * t6519;
    let t9538 = t1538 * t9537;
    let t9539 = t6583 * t9538;
    let t9540 = 0.38342925953920749676e0_f64 * t9539;
    let t9544 = t888 * t2478;
    (t9451, t9454, t9537, t9540, t9544)
}
