//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 336/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk336<F: Float>(t1266: F, t317: F, t1250: F, t1253: F, t1258: F, t1262: F, t313: F) -> (F, F) {
    let t1288 = 11.0 / 9.0 * t317 * t1266;
    let t1289 = 3.0 / 10.0 * t313 * (10.0 / 9.0 * t1250 + 5.0 / 3.0 * t1253 + 10.0 / 9.0 * t1258 + 5.0 / 3.0 * t1262) + t1288;
    (t1288, t1289)
}
