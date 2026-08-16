//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1043/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1043<F: Float>(t1277: F, t37028: F, t263: F, t6660: F, t321: F, t6100: F, t1266: F, t818: F, t826: F, t11033: F, t1289: F, t11050: F, t6654: F) -> (F, F, F, F, F, F, F) {
    let t37029 = t37028 * t1277;
    let t37031 = t263 * t6660;
    let t37038 = t6100 * t321;
    let t37039 = F::cast_from(154.0_f64) / F::cast_from(27.0_f64) * t37038;
    let t37040 = t1266 * t818;
    let t37041 = t37040 * t826;
    let t37043 = t11033 * t1289;
    let t37048 = t6654 * t11050;
    (t37029, t37031, t37039, t37040, t37041, t37043, t37048)
}
