//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 961/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk961<F: Float>(t6739: F, t6742: F, t1796: F, t509: F, t6642: F, t1772: F, t1998: F, t6748: F, t1994: F, t6814: F, t1874: F, t2048: F, t2041: F, t35: F, t88: F, t22338: F, t85: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22704 = t6742 * t6739;
    let t22705 = 0.65061485296689145286e-1 * t22704;
    let t22708 = 0.21687161765563048428e-1 * t1796 * t509 * t6642;
    let t22711 = 0.43374323531126096856e-1 * t1796 * t1772 * t1998;
    let t22712 = t6742 * t6748;
    let t22713 = 0.86748647062252193714e-1 * t22712;
    let t22716 = 0.1284251895870376528e1 * t1796 * t1772 * t1994;
    let t22719 = 0.38527556876111295841e1 * t1796 * t509 * t6814;
    let t22720 = t2048 * t1874;
    let t22721 = 384.0 * t22720;
    let t22723 = t35 * t2041 * t88;
    let t22724 = 1440.0 * t22723;
    let t22726 = 0.19751789702565206229e-1 * t22338 * t85;
    (t22705, t22708, t22711, t22713, t22716, t22719, t22721, t22724, t22726)
}
