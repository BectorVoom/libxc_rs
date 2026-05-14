//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 832/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk832<F: Float>(t6953: F, t6964: F, t7013: F, t7022: F, t7053: F, t7113: F, t7139: F, t8303: F, t6621: F, t990: F, t1249: F, t1248: F, t295: F, t1217: F, t806: F, t1218: F, t2358: F) -> (F, F, F, F, F) {
    let t8306 = t6953 + t6964 + t7013 + t7022 + t7053 + t7113 + t7139 + t8303;
    let t8315 = t6621 * t990;
    let t8316 = t8315 * t1249;
    let t8319 = t295 * t1248;
    let t8320 = t1217 * t806;
    let t8323 = t2358 * t1218;
    (t8306, t8316, t8319, t8320, t8323)
}
