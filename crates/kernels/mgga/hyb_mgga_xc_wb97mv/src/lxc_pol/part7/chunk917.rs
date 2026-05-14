//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 917/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk917<F: Float>(t1861: F, t3014: F, t1978: F, t3: F, t1173: F, t6448: F, t1983: F, t2007: F, t2995: F, t554: F, t2999: F, t24: F, t6811: F) -> (F, F, F, F, F, F, F) {
    let t8451 = t3014 * t1861;
    let t8455 = t1978 * t3;
    let t8459 = t6448 * t1173;
    let t8463 = t1983 * t3;
    let t8469 = t554 * t2007 * t2995 / 96.0;
    let t8472 = t554 * t2007 * t2999 / 96.0;
    let t8473 = t24 * t6811;
    (t8451, t8455, t8459, t8463, t8469, t8472, t8473)
}
