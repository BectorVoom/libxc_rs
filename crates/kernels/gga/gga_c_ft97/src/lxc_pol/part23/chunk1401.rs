//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1401/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1401<F: Float>(t113420: F, t126949: F, t126954: F, t126958: F, t126963: F, t126967: F, t126970: F, t126973: F, t126978: F, t99457: F, t99467: F, t114415: F, t114420: F, t126982: F, t126986: F, t126991: F, t126995: F, t126998: F, t127002: F, t99509: F, t99537: F, t99799: F, t99801: F) -> (F, F) {
    let t128249 = t126949 / 12.0 + 2.0 / 3.0 * t126954 - t126958 + t99457 / 27.0 + 4.0 / 27.0 * t99467 - t126963 / 27.0 + t126967 / 9.0 - 8.0 / 27.0 * t113420 - t126970 / 27.0 - 2.0 / 9.0 * t126973 + t126978 / 12.0;
    let t128258 = -t126982 / 18.0 - 2.0 / 27.0 * t99509 + t99799 + t114415 + t114420 + t126986 / 9.0 + t126991 / 9.0 + t126995 / 9.0 + t99801 - 4.0 / 27.0 * t99537 - 2.0 / 9.0 * t126998 + 4.0 / 3.0 * t127002;
    (t128249, t128258)
}
