//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1065/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1065<F: Float>(t1583: F, t546: F, t2078: F, t3320: F, t783: F, t787: F, t1266: F, t512: F, t57: F, t1607: F, t10856: F, t6271: F) -> (F, F, F, F, F) {
    let t37685 = t546 * t1583;
    let t37696 = t783 * t2078 * t787 * t3320;
    let t37699 = t512 * t1266 * t57;
    let t37700 = t37699 * t1607;
    let t37702 = t10856 * t6271;
    (t37685, t37696, t37699, t37700, t37702)
}
