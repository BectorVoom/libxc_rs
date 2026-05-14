//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1062/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1062<F: Float>(t101767: F, t25904: F, t376: F, t89: F, t25915: F, t25908: F, t1882: F, t25965: F, t1317: F, t25991: F, t3051: F, t5664: F, t1637: F, t6516: F, t6504: F, t25997: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t101768 = t101767 / 18.0;
    let t101771 = t89 * t376 * t25904;
    let t101772 = 4.0 / 9.0 * t101771;
    let t101778 = t89 * t376 * t25915;
    let t101779 = 4.0 / 9.0 * t101778;
    let t101781 = t89 * t376 * t25908;
    let t101782 = 4.0 / 9.0 * t101781;
    let t101811 = t1882 * t25965;
    let t101812 = 4.0 / 3.0 * t101811;
    let t101823 = t1317 * t376 * t25991;
    let t101824 = 2.0 / 9.0 * t101823;
    let t101860 = t5664 * t3051;
    let t101876 = t89 * t1637 * t6516;
    let t101879 = t1317 * t1637 * t6504;
    let t101882 = t1317 * t376 * t25997;
    (t101768, t101771, t101772, t101778, t101779, t101781, t101782, t101811, t101812, t101823, t101824, t101860, t101876, t101879, t101882)
}
