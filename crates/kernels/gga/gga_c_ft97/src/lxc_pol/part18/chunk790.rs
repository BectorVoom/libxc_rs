//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 790/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk790<F: Float>(t11335: F, t72: F, t1669: F, t5586: F, t401: F, t53: F, t1293: F, t77: F) -> (F, F, F, F, F) {
    let t22553 = t72 * t11335;
    let t22557 = t1669 * t5586;
    let t22558 = t401 * t53;
    let t22559 = t72 * t22558;
    let t22563 = t77 * t1293;
    (t22553, t22557, t22558, t22559, t22563)
}
