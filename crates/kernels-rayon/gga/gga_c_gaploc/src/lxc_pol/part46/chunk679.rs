//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 679/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk679(t12411: f64, t295: f64, t3276: f64, t7301: f64, t943: f64, t883: f64, t9603: f64, t7296: f64, t9595: f64, t2562: f64, t2558: f64, t3270: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12580 = t295 * t12411;
    let t12604 = t3276 * t7301;
    let t12605 = t943 * t12604;
    let t12607 = t883 * t9603;
    let t12608 = t7296 * t12607;
    let t12609 = t943 * t12608;
    let t12611 = t883 * t9595;
    let t12612 = t2562 * t12611;
    let t12613 = t943 * t12612;
    let t12623 = t3270 * t2558;
    (t12580, t12604, t12605, t12608, t12609, t12612, t12613, t12623)
}
