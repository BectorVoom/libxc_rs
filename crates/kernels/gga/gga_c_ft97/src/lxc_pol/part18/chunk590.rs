//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 590/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk590<F: Float>(t1802: F, t458: F, t2: F, t8216: F, t1806: F, t3051: F, t94: F, t1771: F, t471: F, t1554: F, t369: F) -> (F, F, F, F, F, F) {
    let t8289 = t458 * t1802;
    let t8291 = t8216 * t2;
    let t8298 = t458 * t1806;
    let t8301 = 28.0 / 27.0 * t3051 * t94;
    let t8302 = t1771 * t471;
    let t8326 = t1554 * t369;
    (t8289, t8291, t8298, t8301, t8302, t8326)
}
