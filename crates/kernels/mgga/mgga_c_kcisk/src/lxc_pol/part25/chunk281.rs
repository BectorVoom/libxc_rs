//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 281/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk281<F: Float>(t1737: F, t1744: F, t1746: F, t1701: F, t1706: F, t1726: F, t1731: F, t1735: F, t45: F, t621: F, t634: F, t67: F, t747: F) -> (F, F, F, F) {
    let t1747 = t1737 * t1744 * t1746;
    let t1750 = -0.62182e-1 * t1701 * t621 + 1.0 * t1706 * t1726 + 0.19751789702565206229e-1 * t45 * t1731 * t634 - 0.58482233974552040708e0 * t1735 * t1747;
    let t1751 = t67 * t1750;
    let t1755 = 1.0 / t747;
    (t1747, t1750, t1751, t1755)
}
