//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 681/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk681(t12608: f64, t943: f64, t883: f64, t9595: f64, t2562: f64, t2558: f64, t3270: f64, t3266: f64, t161: f64, t165: f64, t3234: f64, t2685: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12609 = t943 * t12608;
    let t12611 = t883 * t9595;
    let t12612 = t2562 * t12611;
    let t12613 = t943 * t12612;
    let t12623 = t3270 * t2558;
    let t12624 = t943 * t12623;
    let t12629 = t3266 * t2558;
    let t12630 = t943 * t12629;
    let t12651 = t161 * t165 * t3234;
    let t12652 = t2685 * t12651;
    (t12609, t12612, t12613, t12623, t12624, t12629, t12630, t12651, t12652)
}
