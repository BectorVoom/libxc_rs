//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1320/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1320<F: Float>(t17353: F, t34026: F, t33031: F, t62249: F, t9921: F, t9664: F, t32942: F, t34118: F, t32990: F, t17163: F, t34132: F, t34212: F, t5074: F, t10494: F, t34209: F, t11197: F, t1772: F, t2447: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t116320 = t17353 * t34026;
    let t116321 = t33031 * t116320;
    let t116350 = t62249 * t9921;
    let t116351 = t9664 * t116350;
    let t116368 = 0.23148148148148148148e-2 * t32942 * t34118;
    let t116370 = 0.23148148148148148148e-2 * t32990 * t34118;
    let t116372 = t9664 * t17163 * t34132;
    let t116380 = t5074 * t34212;
    let t116393 = t10494 * t34209;
    let t116394 = 0.3684876543209876543e-2 * t116393;
    let t116409 = t11197 * t2447 * t1772;
    (t116320, t116321, t116350, t116351, t116368, t116370, t116372, t116380, t116393, t116394, t116409)
}
