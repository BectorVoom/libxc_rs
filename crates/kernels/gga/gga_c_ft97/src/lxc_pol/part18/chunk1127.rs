//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1127/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1127<F: Float>(t1882: F, t23894: F, t23926: F, t376: F, t89: F, t23600: F, t23902: F, t1369: F, t1637: F, t5905: F, t5890: F, t5892: F, t23884: F, t358: F, t1636: F, t5925: F) -> (F, F, F, F, F, F, F, F) {
    let t95187 = t1882 * t23894;
    let t95190 = t89 * t376 * t23926;
    let t95205 = t89 * t376 * t23600;
    let t95207 = t1882 * t23902;
    let t95225 = t1369 * t1637 * t5905;
    let t95228 = t5890 * t1637 * t5892;
    let t95234 = t23884 * t358;
    let t95242 = t89 * t1636 * t5925;
    (t95187, t95190, t95205, t95207, t95225, t95228, t95234, t95242)
}
