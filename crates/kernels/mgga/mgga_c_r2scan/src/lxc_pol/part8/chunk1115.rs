//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1115/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1115<F: Float>(t20868: F, t531: F, t147: F, t5134: F, t6103: F, t776: F, t19092: F, t254: F, t261: F, t277: F, t122: F, t20557: F, t507: F, t6326: F, t20481: F, t549: F, t551: F, t560: F) -> (F, F, F, F, F, F) {
    let t20869 = t20868 * t531;
    let t20871 = t5134 * t147;
    let t20916 = t776 * t6103;
    let t20921 = 0.11206619513808432147e2 * t254 * t261 * t19092 * t277;
    let t20925 = 0.32728665637003595454e-5 * t20557 * t6326 * t122 * t507;
    let t20928 = t549 * t551 * t20481 * t560;
    (t20869, t20871, t20916, t20921, t20925, t20928)
}
