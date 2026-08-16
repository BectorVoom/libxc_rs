//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 904/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk904(t447: f64, t9439: f64, t9438: f64, t2476: f64, t475: f64, t587: f64, t40: f64, t599: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9440 = t9439 * t447;
    let t9441 = t9438 * t9440;
    let t9442 = t2476 * t9441;
    let t9444 = t9439 * t475;
    let t9445 = t9438 * t9444;
    let t9446 = t587 * t9445;
    let t9448 = t40 * t599;
    (t9440, t9441, t9442, t9444, t9445, t9446, t9448)
}
