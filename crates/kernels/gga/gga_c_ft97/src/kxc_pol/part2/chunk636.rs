//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 636/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk636<F: Float>(t466: F, t8282: F, t1775: F, t1797: F, t1783: F, t1802: F, t458: F, t2: F, t8216: F, t1806: F, t3051: F, t94: F) -> (F, F, F, F, F, F, F) {
    let t8283 = t8282 * t466;
    let t8285 = t1775 * t1797;
    let t8287 = t1775 * t1783;
    let t8289 = t458 * t1802;
    let t8291 = t8216 * t2;
    let t8298 = t458 * t1806;
    let t8301 = F::new(28.0) / F::new(27.0) * t3051 * t94;
    (t8283, t8285, t8287, t8289, t8291, t8298, t8301)
}
