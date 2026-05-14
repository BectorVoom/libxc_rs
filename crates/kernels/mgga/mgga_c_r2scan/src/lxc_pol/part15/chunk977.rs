//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 977/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk977<F: Float>(t38130: F, t2185: F, t3319: F, t3320: F, t5103: F, t1543: F, t5095: F, t2096: F, t2105: F, t254: F, t265: F, t6079: F, t10868: F, t277: F) -> (F, F, F, F, F) {
    let t38131 = 0.14457274399185490173e-4 * t38130;
    let t38134 = t5103 * t3319 * t3320 * t2185;
    let t38138 = t5095 * t3319 * t3320 * t1543;
    let t38143 = t254 * t6079 * t2096 * t265 * t2105;
    let t38144 = 0.11579802508189808742e1 * t38143;
    let t38145 = t10868 * t277;
    (t38131, t38134, t38138, t38144, t38145)
}
