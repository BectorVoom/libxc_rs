//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 723/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk723<F: Float>(t7440: F, t747: F, t7515: F, t33282: F, t7512: F, t322: F, t626: F) -> (F, F, F, F) {
    let t33283 = t7440 * t747;
    let t33284 = t7515 * t33283;
    let t33286 = t33282 * t7512 * t33284;
    let t33288 = t626 * t322;
    (t33283, t33284, t33286, t33288)
}
