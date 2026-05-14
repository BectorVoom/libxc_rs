//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1023/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1023<F: Float>(t10024: F, t2572: F, t360: F, t2124: F, t8847: F, t921: F, t8811: F, t8773: F, t8832: F, t10010: F, t8837: F, t113: F, t938: F, t8778: F, t2590: F, t1569: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10025 = t2572 * t10024;
    let t10026 = t360 * t10025;
    let t10030 = t2124 * t8847 * t921;
    let t10034 = t2124 * t8811 * t921;
    let t10037 = t8773 * t921;
    let t10038 = t360 * t10037;
    let t10041 = t8832 * t921;
    let t10042 = t360 * t10041;
    let t10046 = t2124 * t8837 * t10010;
    let t10049 = t113 * t938;
    let t10050 = t8778 * t10049;
    let t10051 = t360 * t10050;
    let t10055 = t2124 * t2590 * t10024;
    let t10058 = t1569 * t938;
    (t10025, t10026, t10030, t10034, t10037, t10038, t10041, t10042, t10046, t10049, t10050, t10051, t10055, t10058)
}
