//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 763/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk763<F: Float>(t2601: F, t6135: F, t24432: F, t6118: F, t6108: F, t92: F) -> (F, F, F, F) {
    let t24433 = t6135 * t2601;
    let t24434 = t24432 * t24433;
    let t24435 = t6118 * t24434;
    let t24437 = t6108 * t92;
    (t24433, t24434, t24435, t24437)
}
