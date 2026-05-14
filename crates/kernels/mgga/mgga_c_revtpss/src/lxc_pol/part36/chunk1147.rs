//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1147/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1147<F: Float>(t26024: F, t6846: F, t22061: F, t25986: F, t2661: F, t22026: F, t94550: F, t22056: F, t25972: F, t22021: F, t22068: F, t25978: F, t6880: F, t6856: F, t1904: F, t27985: F, t689: F) -> (F, F, F, F, F, F, F, F, F) {
    let t108592 = t26024 * t6846;
    let t108601 = t2661 * t25986 * t22061;
    let t108604 = t2661 * t94550 * t22026;
    let t108608 = t25972 * t22056;
    let t108623 = t2661 * t25986 * t22021;
    let t108625 = t25972 * t22068;
    let t108627 = t25978 * t6880;
    let t108629 = t25978 * t6856;
    let t108662 = t689 * t27985 * t1904;
    (t108592, t108601, t108604, t108608, t108623, t108625, t108627, t108629, t108662)
}
