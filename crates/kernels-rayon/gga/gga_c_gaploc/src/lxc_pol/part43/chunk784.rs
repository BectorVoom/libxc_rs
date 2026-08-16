//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 784/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk784(t2321: f64, t30301: f64, t9074: f64, t12360: f64, t484: f64, t12352: f64, t2317: f64, t6525: f64, t9061: f64, t30019: f64, t883: f64, t2300: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39731 = t9074 * t30301 * t2321;
    let t39764 = t484 * t12360;
    let t39766 = t484 * t12352;
    let t39774 = t6525 * t9061 * t2317;
    let t39776 = t883 * t30019;
    let t39778 = t9074 * t2300 * t39776;
    (t39731, t39764, t39766, t39774, t39776, t39778)
}
