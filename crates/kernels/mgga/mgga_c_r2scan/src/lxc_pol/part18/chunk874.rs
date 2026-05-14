//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 874/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk874<F: Float>(t10669: F, t11506: F, t3574: F, t481: F, t3263: F, t10610: F, t1100: F, t2881: F, t797: F, t495: F, t3579: F, t3582: F, t3276: F, t3262: F, t106: F, t2530: F, t97: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11507 = t11506 * t10669;
    let t11508 = 3.0 / 4.0 * t11507;
    let t11509 = t3574 * t481;
    let t11510 = t3263 * t11509;
    let t11511 = t10610 * t11510;
    let t11512 = 3.0 / 2.0 * t11511;
    let t11513 = t1100 * t2881;
    let t11514 = t3263 * t797;
    let t11515 = t495 * t11514;
    let t11516 = t3579 * t11515;
    let t11517 = t11516 / 4.0;
    let t11518 = t3582 * t481;
    let t11519 = t3276 * t11518;
    let t11520 = t3262 * t11519;
    let t11521 = 15.0 / 16.0 * t11520;
    let t11523 = t97 * t106 * t2530;
    (t11507, t11508, t11509, t11510, t11511, t11512, t11513, t11515, t11516, t11517, t11518, t11519, t11520, t11521, t11523)
}
