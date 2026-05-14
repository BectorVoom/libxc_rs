//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1093/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1093<F: Float>(t4419: F, t9227: F, t782: F, t12271: F, t24934: F, t5006: F, t12198: F, t22289: F, t12169: F, t10832: F, t22294: F, t5486: F, t18372: F, t5507: F, t9176: F, t1636: F) -> (F, F, F, F, F, F) {
    let t25026 = t4419 * t9227;
    let t25027 = t782 * t25026;
    let t25029 = t12271 * t24934;
    let t25030 = t5006 * t25029;
    let t25033 = t12198 * t22289;
    let t25034 = t5006 * t25033;
    let t25037 = t12169 * t22289;
    let t25038 = t10832 * t25037;
    let t25041 = t5486 * t22294;
    let t25042 = t18372 * t25041;
    let t25045 = t5507 * t9176;
    let t25046 = t25045 * t1636;
    (t25027, t25030, t25034, t25038, t25042, t25046)
}
