//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1285/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1285<F: Float>(t17746: F, t1799: F, t9679: F, t1790: F, t33003: F, t7261: F, t7274: F, t116116: F, t9649: F, t1869: F, t2563: F, t34159: F, t4797: F, t33048: F, t34107: F, t1782: F, t658: F) -> (F, F, F, F, F, F) {
    let t116281 = t1799 * t9679 * t17746;
    let t116285 = t7261 * t33003 * t7274 * t1790;
    let t116289 = 0.26805555555555555556e-2 * t9649 * t116116;
    let t116293 = t1869 * t34159 * t2563 * t4797;
    let t116298 = t1799 * t34107 * t33048;
    let t116304 = t658 * t1782;
    (t116281, t116285, t116289, t116293, t116298, t116304)
}
