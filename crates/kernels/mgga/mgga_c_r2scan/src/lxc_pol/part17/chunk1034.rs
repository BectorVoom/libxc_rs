//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1034/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1034<F: Float>(t10868: F, t6165: F, t9380: F, t24063: F, t30053: F, t3332: F, t6535: F, t9296: F, t12489: F, t25169: F, t10760: F, t19865: F, t30007: F, t261: F, t3299: F, t9366: F) -> (F, F, F, F, F, F) {
    let t43316 = t6165 * t10868 * t9380;
    let t43319 = t24063 * t3332 * t30053;
    let t43322 = t6535 * t10868 * t9296;
    let t43324 = t25169 * t12489;
    let t43327 = t19865 * t10760 * t30007;
    let t43330 = t3299 * t261 * t9366;
    (t43316, t43319, t43322, t43324, t43327, t43330)
}
