//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1047/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1047<F: Float>(t10015: F, t10079: F, t10124: F, t10175: F, t10195: F, t10326: F, t10364: F, t10386: F, t1562: F, t2534: F, t2538: F, t285: F, t3053: F, t3056: F, t499: F, t5087: F, t8714: F, t8723: F, t921: F, t9948: F, t9950: F, t9956: F, t9964: F, t9967: F) -> (F, F) {
    let t10389 = t10015 + t10079 + t10124 + t10175 + t10195 + t10326 + t10364 + t10386;
    let t10392 = t9948 * t285 + 3.0 * t9950 * t2534 + 3.0 / 4.0 * t3053 * t2538 + t9956 * t285 + 3.0 / 4.0 * t3056 * t2538 - 15.0 / 16.0 * t921 * t8714 + 3.0 / 4.0 * t921 * t8723 + 45.0 / 64.0 * t5087 * t9964 - 15.0 / 16.0 * t1562 * t9967 + t499 * t10389 / 4.0;
    (t10389, t10392)
}
