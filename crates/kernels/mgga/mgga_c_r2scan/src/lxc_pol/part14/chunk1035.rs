//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1035/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1035<F: Float>(t10879: F, t11727: F, t261: F, t3304: F, t7309: F, t10760: F, t2147: F, t24059: F, t10740: F, t980: F, t24750: F, t6085: F, t24070: F, t6093: F, t29418: F, t3293: F) -> (F, F, F, F, F, F, F) {
    let t40177 = t10879 * t11727;
    let t40180 = t3304 * t261 * t7309;
    let t40183 = t2147 * t10760 * t24059;
    let t40185 = t980 * t10740;
    let t40188 = t6085 * t10760 * t24750;
    let t40191 = t6093 * t10760 * t24070;
    let t40194 = t3293 * t29418;
    (t40177, t40180, t40183, t40185, t40188, t40191, t40194)
}
