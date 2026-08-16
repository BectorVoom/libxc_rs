//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 881/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk881<F: Float>(t2148: F, t9380: F, t6165: F, t2294: F, t3100: F, t2139: F, t3115: F, t2133: F, t1604: F, t9377: F, t3190: F, t788: F) -> (F, F, F, F, F) {
    let t9381 = t2148 * t9380;
    let t9382 = t6165 * t9381;
    let t9387 = t2294 * t3100;
    let t9388 = t2139 * t9387;
    let t9390 = t2294 * t3115;
    let t9391 = t2133 * t9390;
    let t9397 = t1604 * t9377;
    let t9399 = t788 * t3190;
    (t9382, t9388, t9391, t9397, t9399)
}
