//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1239/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1239<F: Float>(t113402: F, t113406: F, t113411: F, t113415: F, t113421: F, t113423: F, t99457: F, t99467: F, t99770: F, t99776: F, t99783: F, t99784: F, t193: F, t2789: F, t6308: F, t7021: F, t852: F) -> (F, F) {
    let t113425 = -t113402 - t99770 + t113406 / 4.0 + 3.0 / 2.0 * t113411 - 6.0 * t113415 - t99776 + 2.0 / 9.0 * t99457 + 8.0 / 9.0 * t99467 - t99783 + t99784 - t113421 - 4.0 / 9.0 * t113423;
    let t113430 = t6308 * t193 * t852 * t7021 * t2789;
    (t113425, t113430)
}
