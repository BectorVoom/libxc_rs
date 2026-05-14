//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1275/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1275<F: Float>(t113631: F, t113633: F, t113618: F, t113622: F, t113625: F, t113629: F, t99567: F, t99584: F, t99599: F, t99601: F, t99607: F, t99610: F, t114285: F, t114297: F, t114311: F, t114324: F, t114338: F, t114351: F, t114365: F, t114377: F, t114392: F, t114405: F, t114418: F, t114431: F, t114444: F, t114457: F, t114471: F) -> (F,) {
    let t114482 = 4.0 / 27.0 * t113631;
    let t114483 = 4.0 / 27.0 * t113633;
    let t114484 = 4.0 / 3.0 * t113618 + 2.0 / 3.0 * t113622 + 2.0 / 3.0 * t99567 + t99584 / 3.0 - t99599 / 18.0 + 4.0 / 9.0 * t113625 + 4.0 / 27.0 * t99601 + 16.0 / 27.0 * t99607 - 4.0 / 9.0 * t99610 - 4.0 / 9.0 * t113629 + t114482 + t114483;
    let t114488 = t114285 + t114297 + t114311 + t114324 + t114338 + t114351 + t114365 + t114377 + t114392 + t114405 + t114418 + t114431 + t114444 + t114457 + t114471 + t114484;
    (t114488,)
}
