//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 526/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk526<F: Float>(t3106: F, t3109: F, t3141: F, t3160: F, t19: F, t2066: F, t124: F, t1149: F, t329: F, t1152: F, t1140: F, t1156: F, t1133: F, t1117: F, t1137: F, t1121: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3579 = 0.10866666666666666667e1 * t3106;
    let t3580 = 0.978e0 * t3109;
    let t3588 = 0.38033333333333333333e1 * t3141;
    let t3592 = 0.12225e1 * t3160;
    let t3615 = t2066 * t19;
    let t3616 = t124 * t3615;
    let t3621 = t329 * t1149;
    let t3622 = t3621 * t1152;
    let t3624 = t1140 * t1156;
    let t3634 = t1140 * t1133;
    let t3636 = t1137 * t1117;
    let t3638 = t1140 * t1121;
    (t3579, t3580, t3588, t3592, t3616, t3621, t3622, t3624, t3634, t3636, t3638)
}
