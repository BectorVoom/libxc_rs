//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1011/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1011<F: Float>(t23408: F, t925: F, t1969: F, t1882: F, t6632: F, t144: F, t26524: F, t26531: F, t1384: F, t3408: F, t574: F, t605: F, t3578: F, t5947: F, t2142: F, t6699: F) -> (F, F, F, F, F, F, F, F) {
    let t26822 = t23408 * t925;
    let t26823 = t1969 * t26822;
    let t26826 = t1882 * t6632;
    let t26830 = t144 * t26524;
    let t26833 = t144 * t26531;
    let t26836 = t1384 * t3408;
    let t26838 = t574 * t605 * t26836;
    let t26842 = t574 * t3578 * t5947;
    let t26846 = t574 * t2142 * t6699;
    (t26823, t26826, t26830, t26833, t26836, t26838, t26842, t26846)
}
