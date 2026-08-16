//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1190/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1190<F: Float>(t2147: F, t29936: F, t3332: F, t11683: F, t26088: F, t10760: F, t29946: F, t6535: F, t3187: F, t37816: F, t11686: F, t11744: F) -> (F, F, F, F, F) {
    let t43592 = t2147 * t3332 * t29936;
    let t43594 = t26088 * t11683;
    let t43597 = t6535 * t10760 * t29946;
    let t43599 = t37816 * t3187;
    let t43602 = t11744 * t11686;
    (t43592, t43594, t43597, t43599, t43602)
}
