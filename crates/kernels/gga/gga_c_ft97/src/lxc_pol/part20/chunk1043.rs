//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1043/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1043<F: Float>(t99313: F, t99320: F, t99327: F, t99329: F, t99332: F, t99346: F, t99368: F, t99383: F, t99422: F, t99452: F, t99470: F, t99473: F, t99492: F, t99504: F, t99506: F, t99524: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t99735 = t99313 / 3.0;
    let t99739 = 2.0 / 9.0 * t99320;
    let t99741 = 2.0 / 27.0 * t99327;
    let t99742 = t99329 / 9.0;
    let t99743 = t99332 / 6.0;
    let t99747 = 2.0 / 9.0 * t99346;
    let t99754 = t99368 / 9.0;
    let t99759 = 2.0 / 9.0 * t99383;
    let t99770 = t99422 / 12.0;
    let t99776 = t99452 / 3.0;
    let t99783 = 2.0 / 3.0 * t99470;
    let t99784 = t99473 / 8.0;
    let t99789 = t99492 / 9.0;
    let t99793 = t99504 / 18.0;
    let t99794 = t99506 / 27.0;
    let t99799 = 14.0 / 81.0 * t99524;
    (t99735, t99739, t99741, t99742, t99743, t99747, t99754, t99759, t99770, t99776, t99783, t99784, t99789, t99793, t99794, t99799)
}
