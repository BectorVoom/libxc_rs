//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 627/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk627<F: Float>(t9162: F, t9257: F, t605: F, t144: F, t167: F, t574: F, t9007: F, t2075: F, t616: F, t576: F, t8232: F, t611: F, t1882: F, t2174: F, t2178: F, t597: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9258 = t9162 + t9257;
    let t9259 = t605 * t9258;
    let t9260 = t144 * t9259;
    let t9264 = t574 * t167 * t9007;
    let t9268 = t574 * t616 * t2075;
    let t9270 = t8232 * t576;
    let t9272 = t8232 * t611;
    let t9274 = t1882 * t2174;
    let t9276 = t597 * t2178;
    (t9258, t9259, t9260, t9264, t9268, t9270, t9272, t9274, t9276)
}
