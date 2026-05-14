//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1019/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1019<F: Float>(t18000: F, t7034: F, t140: F, t2554: F, t430: F, t11916: F, t1919: F, t2063: F, t220: F, t5254: F, t299: F, t7394: F, t3293: F, t7389: F, t2505: F, t4597: F) -> (F, F, F, F, F, F, F) {
    let t18001 = t7034 * t18000;
    let t18005 = t140 * t430 * t2554;
    let t18022 = t1919 * t11916 * t2063;
    let t18026 = t1919 * t5254 * t220;
    let t18031 = 0.53062222222222222222e-1 * t140 * t299 * t7394;
    let t18033 = t1919 * t7389 * t3293;
    let t18036 = t2505 * t4597;
    (t18001, t18005, t18022, t18026, t18031, t18033, t18036)
}
