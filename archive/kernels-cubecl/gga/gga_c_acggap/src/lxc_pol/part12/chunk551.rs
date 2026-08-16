//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 551/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk551<F: Float>(t3732: F, t384: F, t377: F, t951: F, t409: F, t1086: F, t997: F, t1032: F, t1113: F, t1092: F, t1098: F, t1108: F) -> (F, F, F, F, F, F, F, F) {
    let t3733 = t384 * t3732;
    let t3740 = t377 * t951;
    let t3741 = t3740 * t409;
    let t3743 = t997 * t1086;
    let t3745 = t1032 * t1113;
    let t3747 = t997 * t1092;
    let t3750 = t997 * t1098;
    let t3752 = t1032 * t1108;
    (t3733, t3740, t3741, t3743, t3745, t3747, t3750, t3752)
}
