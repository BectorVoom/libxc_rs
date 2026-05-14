//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 873/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk873<F: Float>(t11497: F, t3263: F, t3262: F, t1065: F, t910: F, t3270: F, t10667: F, t105: F, t920: F, t97: F) -> (F, F, F, F, F, F, F, F) {
    let t11498 = t3263 * t11497;
    let t11499 = t3262 * t11498;
    let t11500 = 3.0 / 4.0 * t11499;
    let t11501 = t1065 * t910;
    let t11502 = t3270 * t11501;
    let t11503 = t10667 * t11502;
    let t11504 = 3.0 / 4.0 * t11503;
    let t11505 = t105 * t920;
    let t11506 = t97 * t11505;
    (t11498, t11499, t11500, t11502, t11503, t11504, t11505, t11506)
}
