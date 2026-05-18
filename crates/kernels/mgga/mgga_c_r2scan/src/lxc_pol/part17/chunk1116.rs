//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1116/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1116<F: Float>(t10903: F, t11764: F, t2207: F, t261: F, t3299: F, t7390: F, t10879: F, t11727: F, t3304: F, t7309: F, t10740: F, t980: F) -> (F, F, F, F, F) {
    let t40162 = t2207 * t10903 * t11764;
    let t40175 = t3299 * t261 * t7390;
    let t40177 = t10879 * t11727;
    let t40180 = t3304 * t261 * t7309;
    let t40185 = t980 * t10740;
    (t40162, t40175, t40177, t40180, t40185)
}
