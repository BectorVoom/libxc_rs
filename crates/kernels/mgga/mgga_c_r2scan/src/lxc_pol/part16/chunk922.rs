//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 922/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk922<F: Float>(t12582: F, t1108: F, t2995: F, t1100: F, t3250: F, t11036: F, t2928: F, t2938: F, t3358: F, t1070: F, t9640: F, t3629: F, t8358: F, t6661: F, t1276: F, t11032: F, t11058: F, t12230: F, t12235: F, t12238: F) -> (F, F, F, F, F, F) {
    let t12583 = 2.0 * t12582;
    let t12584 = t2995 * t1108;
    let t12585 = t1100 * t3250;
    let t12587 = t11036 * t2928;
    let t12589 = t3358 * t2938;
    let t12591 = t9640 * t1070;
    let t12593 = t8358 * t3629;
    let t12595 = t1070 * t2928;
    let t12596 = t6661 * t12595;
    let t12598 = t1070 * t2938;
    let t12599 = t1276 * t12598;
    let t12601 = -t11032 - t12230 - t12587 / 4.0 + t12589 / 8.0 - t12591 / 8.0 + t12593 / 2.0 + t12235 - 3.0 / 4.0 * t12596 - t12238 + t12599 / 4.0 - t11058;
    (t12583, t12584, t12585, t12595, t12598, t12601)
}
