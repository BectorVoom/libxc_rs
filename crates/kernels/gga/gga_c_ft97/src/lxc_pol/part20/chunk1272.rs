//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1272/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1272<F: Float>(t113402: F, t113406: F, t113411: F, t113415: F, t113420: F, t113423: F, t99422: F, t99452: F, t99457: F, t99467: F, t99470: F, t99473: F, t113446: F, t113430: F, t113434: F, t113439: F, t113443: F, t113450: F, t113453: F, t99492: F, t99504: F, t99506: F, t99509: F, t99799: F) -> (F, F) {
    let t114405 = -t113402 / 3.0 - t99422 / 36.0 + t113406 / 12.0 + t113411 / 2.0 - 2.0 * t113415 - t99452 / 9.0 + 2.0 / 27.0 * t99457 + 8.0 / 27.0 * t99467 - 2.0 / 9.0 * t99470 + t99473 / 24.0 - 4.0 / 27.0 * t113420 - 4.0 / 27.0 * t113423;
    let t114415 = 2.0 / 9.0 * t113446;
    let t114418 = t113430 / 12.0 + 4.0 * t113434 + t113439 / 4.0 - t113443 / 3.0 - t99492 / 27.0 - t99504 / 54.0 - t99506 / 81.0 - 4.0 / 27.0 * t99509 + t99799 + t114415 - t113450 / 3.0 + t113453 / 9.0;
    (t114405, t114418)
}
