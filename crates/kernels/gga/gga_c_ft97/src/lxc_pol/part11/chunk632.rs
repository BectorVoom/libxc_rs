//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 632/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk632<F: Float>(t2210: F, t9354: F, t3434: F, t8376: F, t3440: F, t3439: F, t8805: F, t9065: F, t9068: F, t8796: F, t8799: F, t8802: F, t9010: F, t9020: F, t9035: F, t9039: F, t9043: F, t9047: F, t9052: F) -> (F, F, F, F, F, F) {
    let t9355 = t2210 * t9354;
    let t9358 = t3434 * t8376;
    let t9359 = t2210 * t9358;
    let t9362 = t3440 * t8376;
    let t9363 = t3439 * t9362;
    let t9366 = 2.0 / 3.0 * t8805;
    let t9369 = 4.0 / 9.0 * t9065;
    let t9370 = t9068 / 3.0;
    let t9371 = 4.0 / 27.0 * t8796;
    let t9372 = t8799 / 9.0;
    let t9373 = 2.0 / 27.0 * t8802;
    let t9379 = -t9366 - t9010 / 3.0 - 2.0 * t9020 - t9369 + t9370 - t9371 + t9372 + t9373 + 2.0 / 3.0 * t9035 - 2.0 / 9.0 * t9039 + t9043 / 3.0 + t9047 / 3.0 + 2.0 / 9.0 * t9052;
    (t9355, t9358, t9359, t9362, t9363, t9379)
}
