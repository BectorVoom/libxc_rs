//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 888/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk888<F: Float>(t1554: F, t2562: F, t360: F, t2567: F, t113: F, t2185: F, t2572: F, t2719: F, t560: F, t551: F, t552: F, t538: F, t920: F) -> (F, F, F, F, F, F, F, F) {
    let t8102 = t2562 * t1554;
    let t8103 = t360 * t8102;
    let t8106 = t2567 * t1554;
    let t8107 = t360 * t8106;
    let t8110 = t113 * t2185;
    let t8111 = t2572 * t8110;
    let t8112 = t360 * t8111;
    let t8117 = t2719 * t560;
    let t8119 = t551 * t552 * t8117;
    let t8123 = t538 * t920;
    (t8102, t8103, t8106, t8107, t8111, t8112, t8119, t8123)
}
