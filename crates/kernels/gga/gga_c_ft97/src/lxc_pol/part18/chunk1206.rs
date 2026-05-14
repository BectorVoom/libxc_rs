//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1206/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1206<F: Float>(t1882: F, t25965: F, t101621: F, t1564: F, t446: F, t101626: F, t7793: F, t100370: F, t7824: F, t1317: F, t25991: F, t376: F, t26001: F, t432: F, t8411: F, t11392: F, t28: F, t5507: F, t89: F) -> (F, F, F, F, F, F, F, F, F) {
    let t101811 = t1882 * t25965;
    let t101812 = 4.0 / 3.0 * t101811;
    let t101814 = t446 * t1564 * t101621;
    let t101817 = t446 * t7793 * t101626;
    let t101820 = t446 * t7824 * t100370;
    let t101823 = t1317 * t376 * t25991;
    let t101824 = 2.0 / 9.0 * t101823;
    let t101827 = t446 * t8411 * t26001 * t432;
    let t101831 = t89 * t28 * t5507 * t11392;
    (t101811, t101812, t101814, t101817, t101820, t101823, t101824, t101827, t101831)
}
