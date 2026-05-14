//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1009/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1009<F: Float>(t97029: F, t24555: F, t681: F, t89: F, t2399: F, t6140: F, t683: F, t9942: F, t24487: F, t375: F, t24494: F, t24490: F, t1434: F, t6124: F, t24466: F, t24461: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t97030 = 14.0 / 27.0 * t97029;
    let t97046 = t89 * t681 * t24555;
    let t97061 = t89 * t2399 * t6140;
    let t97078 = t683 * t9942;
    let t97084 = t89 * t375 * t24487;
    let t97089 = t89 * t681 * t24494;
    let t97092 = t89 * t681 * t24490;
    let t97123 = t1434 * t2399 * t6124;
    let t97144 = t1434 * t681 * t24466;
    let t97154 = t1434 * t681 * t24461;
    (t97030, t97046, t97061, t97078, t97084, t97089, t97092, t97123, t97144, t97154)
}
