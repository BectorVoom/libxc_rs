//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 965/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk965<F: Float>(t1160: F, t14575: F, t1629: F, t1035: F, t4225: F, t864: F, t3646: F, t550: F, t1636: F, t980: F, t3378: F, t4194: F, t3088: F, t4166: F, t4183: F, t4176: F) -> (F, F, F, F, F, F, F) {
    let t19032 = t1160 * t1629 * t14575;
    let t19038 = t1035 * t4225 * t864;
    let t19040 = t3646 * t550;
    let t19042 = t980 * t1636;
    let t19045 = t3378 * t4194;
    let t19048 = t3088 * t4166 * t4183;
    let t19053 = t3378 * t4176;
    (t19032, t19038, t19040, t19042, t19045, t19048, t19053)
}
