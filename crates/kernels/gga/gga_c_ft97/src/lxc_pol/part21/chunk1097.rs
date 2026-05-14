//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1097/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1097<F: Float>(t105765: F, t1882: F, t27103: F, t27100: F, t1359: F, t40424: F, t27078: F, t95053: F, t1369: F, t27047: F, t376: F, t1557: F, t6615: F, t27031: F, t23608: F, t27160: F, t458: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t105766 = 4.0 / 9.0 * t105765;
    let t105770 = t1882 * t27103;
    let t105771 = 4.0 / 27.0 * t105770;
    let t105772 = t1882 * t27100;
    let t105773 = 4.0 / 9.0 * t105772;
    let t105797 = t40424 * t1359;
    let t105809 = t95053 * t27078;
    let t105810 = t105809 / 18.0;
    let t105815 = t1369 * t376 * t27047;
    let t105816 = t105815 / 3.0;
    let t105821 = t6615 * t1557;
    let t105826 = t1882 * t27031;
    let t105827 = 2.0 / 9.0 * t105826;
    let t105846 = t23608 * t458 * t27160;
    (t105766, t105770, t105771, t105772, t105773, t105797, t105809, t105810, t105815, t105816, t105821, t105826, t105827, t105846)
}
