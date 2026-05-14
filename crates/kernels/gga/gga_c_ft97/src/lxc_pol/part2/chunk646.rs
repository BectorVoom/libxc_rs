//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 646/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk646<F: Float>(t11356: F, t371: F, t122: F, t409: F, t1751: F, t374: F, t930: F, t3021: F, t401: F, t7983: F, t408: F, t929: F, t3020: F, t428: F, t388: F, t939: F) -> (F, F, F, F, F, F) {
    let t11357 = t371 * t11356;
    let t11360 = t409 * t122;
    let t11361 = t371 * t11360;
    let t11368 = t374 * t930 * t1751;
    let t11371 = t3021 * t401;
    let t11372 = t7983 * t11371;
    let t11375 = t408 * t929;
    let t11377 = t3020 * t11375 * t428;
    let t11380 = t388 * t939;
    (t11357, t11361, t11368, t11372, t11377, t11380)
}
