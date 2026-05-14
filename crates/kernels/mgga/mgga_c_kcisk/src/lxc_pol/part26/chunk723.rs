//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 723/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk723<F: Float>(t1053: F, t2692: F, t3186: F, t1032: F, t167: F, t1001: F, t1035: F, t206: F, t967: F, t1042: F, t2689: F, t982: F, t1049: F, t116: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9340 = t2692 * t1053;
    let t9341 = t3186 * t9340;
    let t9342 = 2.0 * t9341;
    let t9343 = t1032 * t167;
    let t9345 = t167 * t1001;
    let t9346 = t1035 * t9345;
    let t9348 = t206 * t967;
    let t9350 = t1042 * t2689;
    let t9352 = t2689 * t1001;
    let t9353 = t982 * t9352;
    let t9355 = t116 * t1049;
    (t9340, t9342, t9343, t9345, t9346, t9348, t9350, t9352, t9353, t9355)
}
