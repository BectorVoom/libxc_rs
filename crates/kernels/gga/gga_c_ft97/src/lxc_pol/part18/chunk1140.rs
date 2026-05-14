//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1140/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1140<F: Float>(t1882: F, t23982: F, t1378: F, t9114: F, t1386: F, t3281: F, t23560: F, t8392: F, t23487: F, t24152: F, t5: F, t26513: F, t27462: F, t108: F, t7800: F, t358: F, t984: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t96232 = t1882 * t23982;
    let t96239 = t9114 * t1378;
    let t96244 = 28.0 / 81.0 * t3281 * t1386;
    let t96251 = t8392 * t23560;
    let t96269 = t1882 * t23487;
    let t96310 = t5 * t24152;
    let t100044 = 2.0 * t26513;
    let t100045 = 2.0 * t27462;
    let t100050 = t108 * t7800;
    let t100055 = t984 * t358;
    (t96232, t96239, t96244, t96251, t96269, t96310, t100044, t100045, t100050, t100055)
}
