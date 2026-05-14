//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1141/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1141<F: Float>(t5147: F, t5148: F, t5173: F, t2195: F, t6486: F, t2183: F, t10979: F, t110: F, t20420: F, t19862: F, t1605: F, t6188: F, t6189: F, t481: F, t5: F, t7: F) -> (F, F, F, F, F, F, F) {
    let t20782 = t5147 * t5148 * t5173;
    let t20784 = t2195 * t6486;
    let t20787 = t2183 * t6486;
    let t20791 = t20420 * t10979 * t110;
    let t20792 = t20791 * t19862;
    let t20818 = t6188 * t6189 * t1605;
    let t20820 = t5 * t7 * t481;
    (t20782, t20784, t20787, t20791, t20792, t20818, t20820)
}
