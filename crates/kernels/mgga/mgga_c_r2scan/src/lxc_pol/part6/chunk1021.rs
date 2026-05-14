//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1021/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1021<F: Float>(t2061: F, t7902: F, t6030: F, t6033: F, t7108: F, t7110: F, t7112: F, t7126: F, t7128: F, t7149: F, t7150: F, t7898: F, t246: F, t4873: F, t5032: F, t5039: F, t6036: F, t6039: F, t6047: F, t7028: F, t7156: F, t7158: F, t7160: F, t7161: F) -> (F, F) {
    let t7904 = 0.1350520664e0 * t2061 * t7902;
    let t7905 = t7108 - t7110 - t7112 - 0.571528e-1 * t7898 + 0.2701041328e0 * t6030 - 0.675260332e-1 * t6033 - t7126 - t7128 - t7904 + t7149 + t7150;
    let t7910 = -t4873 + 0.285764e-1 * t6036 + 0.571528e-1 * t6039 + t6047 - 0.285764e-1 * t246 * t7028 + t7156 + t7158 + t7160 - t5032 - t7161 - t5039;
    (t7905, t7910)
}
