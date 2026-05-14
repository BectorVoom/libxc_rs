//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 948/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk948<F: Float>(t795: F, t986: F, t113: F, t5086: F, t104: F, t494: F, t1275: F, t502: F, t1277: F, t263: F, t6660: F, t321: F, t6100: F, t1266: F, t818: F, t826: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32212 = t986 * t795;
    let t36967 = t113 * t5086;
    let t36985 = t104 * t494;
    let t37028 = t502 * t1275;
    let t37029 = t37028 * t1277;
    let t37031 = t263 * t6660;
    let t37038 = t6100 * t321;
    let t37040 = t1266 * t818;
    let t37041 = t37040 * t826;
    (t32212, t36967, t36985, t37028, t37029, t37031, t37038, t37040, t37041)
}
