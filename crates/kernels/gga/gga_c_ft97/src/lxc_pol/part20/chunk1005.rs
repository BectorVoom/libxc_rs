//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1005/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1005<F: Float>(t24220: F, t5996: F, t1403: F, t24397: F, t681: F, t24256: F, t92: F, t458: F, t6108: F) -> (F, F, F, F) {
    let t96854 = t5996 * t24220;
    let t96857 = t1403 * t681 * t24397;
    let t96863 = t24256 * t92;
    let t96925 = t6108 * t458;
    (t96854, t96857, t96863, t96925)
}
