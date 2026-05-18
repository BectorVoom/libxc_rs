//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 498/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk498<F: Float>(t912: F, t9278: F, t587: F, t544: F, t6603: F, t107: F, t90: F, t2321: F) -> (F, F, F, F) {
    let t9279 = t912 * t9278;
    let t9280 = t587 * t9279;
    let t9281 = F::new(0.38342925953920749676e0) * t9280;
    let t9285 = t544 * t6603;
    let t9286 = t107 * t90;
    let t9287 = t9286 * t2321;
    (t9281, t9285, t9286, t9287)
}
