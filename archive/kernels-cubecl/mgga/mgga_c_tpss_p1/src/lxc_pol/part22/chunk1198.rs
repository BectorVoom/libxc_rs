//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1198/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1198<F: Float>(t18394: F, t640: F, t2073: F, t68: F, t2074: F, t2100: F, t5527: F, t1695: F, t17942: F, t510: F, t517: F, t5543: F) -> (F, F, F, F, F, F, F) {
    let t18395 = t18394 * t640;
    let t18396 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t18395;
    let t18397 = t68 * t2073;
    let t18398 = t18397 * t2074;
    let t18400 = t5527 * t2100;
    let t18434 = t17942 * t510 * t1695;
    let t18436 = t5543 * t517;
    (t18395, t18396, t18397, t18398, t18400, t18434, t18436)
}
