//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 900/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk900<F: Float>(t11497: F, t3263: F, t3262: F, t1065: F, t910: F, t3270: F, t10667: F, t105: F, t920: F, t97: F, t10669: F, t3574: F, t481: F, t10610: F, t1100: F, t2881: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11498 = t3263 * t11497;
    let t11499 = t3262 * t11498;
    let t11500 = 3.0 / 4.0 * t11499;
    let t11501 = t1065 * t910;
    let t11502 = t3270 * t11501;
    let t11503 = t10667 * t11502;
    let t11504 = 3.0 / 4.0 * t11503;
    let t11505 = t105 * t920;
    let t11506 = t97 * t11505;
    let t11507 = t11506 * t10669;
    let t11508 = 3.0 / 4.0 * t11507;
    let t11509 = t3574 * t481;
    let t11510 = t3263 * t11509;
    let t11511 = t10610 * t11510;
    let t11512 = 3.0 / 2.0 * t11511;
    let t11513 = t1100 * t2881;
    (t11498, t11499, t11500, t11502, t11503, t11504, t11505, t11506, t11507, t11508, t11509, t11510, t11511, t11512, t11513)
}
