//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1027/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1027<F: Float>(t1456: F, t4969: F, t724: F, t5053: F, t729: F, t1175: F, t6837: F, t242: F, t30946: F, t30942: F, t265: F, t30859: F, t2574: F, t4934: F, t24569: F, t5165: F) -> (F, F, F, F, F, F, F, F) {
    let t31273 = t724 * t1456 * t4969;
    let t31279 = t729 * t1456 * t5053;
    let t31283 = t729 * t1175 * t6837;
    let t31286 = t242 * t30946;
    let t31289 = t242 * t30942;
    let t31293 = t729 * t265 * t30859;
    let t31298 = t2574 * t1456 * t4934;
    let t31302 = t24569 * t5165;
    (t31273, t31279, t31283, t31286, t31289, t31293, t31298, t31302)
}
