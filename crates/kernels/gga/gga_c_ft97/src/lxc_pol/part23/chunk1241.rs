//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1241/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1241<F: Float>(t10157: F, t24437: F, t4934: F, t6119: F, t747: F, t16579: F, t2354: F, t446: F, t6135: F, t4917: F, t97190: F, t9744: F, t1434: F, t30996: F, t681: F, t1882: F, t30975: F) -> (F, F, F, F, F) {
    let t123859 = t24437 * t10157 * t6119 * t4934 * t747;
    let t123863 = t446 * t2354 * t6135 * t16579;
    let t123867 = t446 * t9744 * t97190 * t4917;
    let t123870 = t1434 * t681 * t30996;
    let t123872 = t1882 * t30975;
    (t123859, t123863, t123867, t123870, t123872)
}
