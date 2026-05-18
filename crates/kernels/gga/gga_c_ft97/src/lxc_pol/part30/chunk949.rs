//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 949/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk949<F: Float>(t1403: F, t2336: F, t33546: F, t2252: F, t342: F, t7430: F, t33561: F, t630: F, t24499: F, t24220: F, t7437: F, t33583: F, t681: F) -> (F, F, F, F, F, F) {
    let t141478 = t1403 * t2336 * t33546;
    let t141489 = t342 * t2252 * t7430 / F::new(18.0);
    let t141491 = t342 * t630 * t33561;
    let t141509 = t1403 * t24499;
    let t141524 = t7437 * t24220;
    let t141527 = t1403 * t681 * t33583;
    (t141478, t141489, t141491, t141509, t141524, t141527)
}
