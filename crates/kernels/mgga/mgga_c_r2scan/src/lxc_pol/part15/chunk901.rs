//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 901/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk901<F: Float>(t3263: F, t797: F, t495: F, t3579: F, t3582: F, t481: F, t3276: F, t3262: F, t106: F, t2530: F, t97: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11514 = t3263 * t797;
    let t11515 = t495 * t11514;
    let t11516 = t3579 * t11515;
    let t11517 = t11516 / 4.0;
    let t11518 = t3582 * t481;
    let t11519 = t3276 * t11518;
    let t11520 = t3262 * t11519;
    let t11521 = 15.0 / 16.0 * t11520;
    let t11523 = t97 * t106 * t2530;
    (t11514, t11515, t11516, t11517, t11518, t11519, t11520, t11521, t11523)
}
