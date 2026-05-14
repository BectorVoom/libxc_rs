//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1048/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1048<F: Float>(t7240: F, t81: F, t39: F, t8051: F, t8907: F, t142: F, t7367: F, t71: F, t938: F) -> (F, F, F, F, F) {
    let t32075 = 1.0 / t7240 / t81;
    let t32167 = t8051 * t39;
    let t32772 = t8907 * t39;
    let t32905 = 1.0 / t7367 / t142;
    let t34433 = t71 * t938;
    (t32075, t32167, t32772, t32905, t34433)
}
