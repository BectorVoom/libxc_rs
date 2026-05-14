//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 685/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk685<F: Float>(t33489: F, t762: F, t263: F, t7440: F, t684: F, t9770: F, t7436: F, t92: F) -> (F, F, F, F) {
    let t33490 = t762 * t33489;
    let t33494 = t7440 * t263;
    let t33496 = t9770 * t33494 * t684;
    let t33499 = t7436 * t92;
    (t33490, t33494, t33496, t33499)
}
