//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 562/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk562<F: Float>(t287: F, t2909: F, t1003: F, t286: F, t237: F, t240: F, t334: F) -> (F, F, F, F) {
    let t2910 = t287 * t2909;
    let t2911 = t1003 * t1003;
    let t2912 = t2910 * t2911;
    let t2913 = t286 * t2912;
    let t2917 = t237 * t334 * t240;
    (t2911, t2912, t2913, t2917)
}
