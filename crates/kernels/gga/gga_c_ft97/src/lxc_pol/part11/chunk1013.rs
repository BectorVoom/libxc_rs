//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1013/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1013<F: Float>(t10666: F, t2801: F, t10738: F, t1882: F, t10741: F, t192: F, t33828: F, t10714: F, t2399: F, t2834: F, t89: F, t2751: F, t8232: F, t10690: F, t2832: F, t848: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44272 = t10666 * t2801;
    let t44276 = t1882 * t10738;
    let t44278 = t1882 * t10741;
    let t44280 = t192 * t33828;
    let t44289 = t1882 * t10714;
    let t44292 = t89 * t2399 * t2834;
    let t44294 = t8232 * t2751;
    let t44300 = t1882 * t10690;
    let t44302 = t848 * t2832;
    (t44272, t44276, t44278, t44280, t44289, t44292, t44294, t44300, t44302)
}
