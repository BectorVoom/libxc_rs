//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 782/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk782<F: Float>(t135: F, t9105: F, t4082: F, t4085: F, t1247: F, t2282: F, t12399: F, t467: F, t871: F, t9117: F, t2287: F, t3113: F) -> (F, F, F, F, F, F) {
    let t39644 = t135 * t9105;
    let t39646 = t4082 * t39644 * t4085;
    let t39648 = t1247 * t2282;
    let t39650 = t12399 * t467;
    let t39656 = t9117 * t871;
    let t39657 = t2287 * t3113;
    (t39644, t39646, t39648, t39650, t39656, t39657)
}
