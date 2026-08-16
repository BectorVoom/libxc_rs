//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 130/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk130(t475: f64, t569: f64, t568: f64, t200: f64, t8: f64, t203: f64, t61: f64, t120: f64, t196: f64) -> (f64, f64, f64, f64) {
    let t575 = t569 * t475;
    let t576 = t568 * t575;
    let t579 = t8 * t200;
    let t580 = t579 * t203;
    let t581 = t61 * t580;
    let t584 = t196 * t120;
    (t576, t579, t581, t584)
}
