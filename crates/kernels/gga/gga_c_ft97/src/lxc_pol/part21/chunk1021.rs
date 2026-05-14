//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1021/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1021<F: Float>(t1602: F, t409: F, t39: F, t5585: F, t1613: F, t5589: F, t420: F, t5571: F, t411: F, t5576: F, t415: F, t5523: F, t7983: F, t172: F, t22766: F, t17839: F, t5566: F) -> (F, F, F, F, F, F, F, F, F) {
    let t92353 = t1602 * t409;
    let t92354 = t39 * t5585;
    let t92356 = t92354 * t1613 * t5589;
    let t92358 = t420 * t5571;
    let t92367 = t5576 * t411;
    let t92370 = t5576 * t415;
    let t92414 = t7983 * t5523;
    let t92433 = t22766 * t172;
    let t92439 = t5566 * t17839;
    (t92353, t92354, t92356, t92358, t92367, t92370, t92414, t92433, t92439)
}
