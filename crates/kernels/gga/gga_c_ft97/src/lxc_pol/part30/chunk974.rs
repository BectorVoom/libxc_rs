//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 974/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk974<F: Float>(t2360: F, t7611: F, t2680: F, t33953: F, t33967: F, t681: F, t89: F, t33971: F, t33984: F, t1882: F, t33963: F, t1486: F, t2399: F, t7646: F) -> (F, F, F, F, F, F, F) {
    let t143284 = t7611 * t2360;
    let t143293 = t2680 * t33953;
    let t143321 = t89 * t681 * t33967;
    let t143324 = t89 * t681 * t33971;
    let t143327 = t89 * t681 * t33984;
    let t143329 = t1882 * t33963;
    let t143332 = t1486 * t2399 * t7646;
    (t143284, t143293, t143321, t143324, t143327, t143329, t143332)
}
