//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 914/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk914<F: Float>(t16009: F, t1835: F, t16013: F, t1659: F, t15999: F, t4726: F, t6891: F, t970: F, t16017: F, t1856: F, t6894: F, t1842: F, t16026: F, t16004: F, t706: F, t6928: F, t960: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t16153 = t1835 * t16009;
    let t16156 = t1659 * t16013;
    let t16159 = t4726 * t15999;
    let t16163 = 0.4705225e-4 * t970 * t6891;
    let t16164 = t1856 * t16017;
    let t16167 = t970 * t6894;
    let t16169 = t1842 * t16017;
    let t16172 = t1835 * t16017;
    let t16175 = t1856 * t16026;
    let t16178 = t1835 * t16004;
    let t16181 = t1835 * t16013;
    let t16184 = t706 * t15999;
    let t16188 = 0.18736e-1 * t960 * t6928;
    (t16153, t16156, t16159, t16163, t16164, t16167, t16169, t16172, t16175, t16178, t16181, t16184, t16188)
}
