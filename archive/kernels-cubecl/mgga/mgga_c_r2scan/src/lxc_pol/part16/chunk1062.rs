//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1062/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1062<F: Float>(t37637: F, t37638: F, t1583: F, t565: F, t2195: F, t573: F, t10707: F, t1591: F, t546: F, t1266: F, t512: F, t57: F) -> (F, F, F, F, F, F) {
    let t37639 = t37637 * t37638;
    let t37641 = t565 * t1583;
    let t37652 = t2195 * t573;
    let t37658 = t1591 * t10707;
    let t37685 = t546 * t1583;
    let t37699 = t512 * t1266 * t57;
    (t37639, t37641, t37652, t37658, t37685, t37699)
}
