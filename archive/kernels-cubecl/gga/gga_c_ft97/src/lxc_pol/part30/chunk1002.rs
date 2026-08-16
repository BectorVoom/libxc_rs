//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1002/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1002<F: Float>(t150038: F, t446: F, t9770: F, t33288: F, t35319: F, t7511: F, t6061: F, t6837: F, t7512: F, t7515: F, t1434: F, t35336: F, t681: F) -> (F, F, F, F, F) {
    let t150128 = t446 * t9770 * t150038;
    let t150131 = t7511 * t33288 * t35319;
    let t150133 = t6061 * t6837;
    let t150136 = t7511 * t7512 * t7515 * t150133;
    let t150139 = t1434 * t681 * t35336;
    (t150128, t150131, t150133, t150136, t150139)
}
