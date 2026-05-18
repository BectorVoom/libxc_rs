//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 656/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk656<F: Float>(t611: F, t8232: F, t1882: F, t2174: F, t2178: F, t597: F, t2135: F, t376: F, t89: F, t571: F, t2192: F, t2207: F) -> (F, F, F, F, F, F, F) {
    let t9272 = t8232 * t611;
    let t9274 = t1882 * t2174;
    let t9276 = t597 * t2178;
    let t9282 = t89 * t376 * t2135;
    let t9298 = t8232 * t571;
    let t9300 = t1882 * t2192;
    let t9302 = t1882 * t2207;
    (t9272, t9274, t9276, t9282, t9298, t9300, t9302)
}
