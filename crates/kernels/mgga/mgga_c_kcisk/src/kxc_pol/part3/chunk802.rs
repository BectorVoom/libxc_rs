//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 802/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk802<F: Float>(t13336: F, t3785: F, t1411: F, t12957: F, t1341: F, t1340: F, t1339: F, t3748: F, t3770: F, t3512: F, t3769: F, t3583: F, t3764: F, t3575: F, t3759: F, t12952: F) -> (F, F, F, F, F, F, F) {
    let t13337 = t3785 * t13336;
    let t13338 = t1411 * t13337;
    let t13340 = t1341 * t12957;
    let t13341 = t1340 * t13340;
    let t13342 = t1339 * t13341;
    let t13344 = t3748 * t3770;
    let t13346 = t3512 * t3769;
    let t13347 = t1339 * t13346;
    let t13349 = t3764 * t3583;
    let t13350 = t1340 * t13349;
    let t13351 = t1339 * t13350;
    let t13353 = t3764 * t3575;
    let t13354 = t1340 * t13353;
    let t13355 = t3759 * t13354;
    let t13357 = t1341 * t12952;
    (t13338, t13342, t13344, t13347, t13351, t13355, t13357)
}
