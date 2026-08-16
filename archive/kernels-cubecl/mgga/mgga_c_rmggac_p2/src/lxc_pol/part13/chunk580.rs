//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 580/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk580<F: Float>(t36: F, t5245: F, t344: F, t830: F, t3839: F, t7634: F, t1243: F, t128: F, t118: F, t2001: F, t675: F, t1987: F, t2191: F) -> (F, F, F, F, F, F) {
    let t7660 = t5245 * t36;
    let t7662 = t344 * t830;
    let t7664 = t3839 * t7634;
    let t7675 = t128 * t1243;
    let t7676 = t118 * t7675;
    let t7677 = t2001 * t7676;
    let t7678 = t675 * t7677;
    let t7680 = t2191 * t1987;
    (t7660, t7662, t7664, t7677, t7678, t7680)
}
