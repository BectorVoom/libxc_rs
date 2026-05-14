//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 721/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk721<F: Float>(t7491: F, t8355: F, t8349: F, t2416: F, t35704: F, t2160: F, t638: F, t8858: F, t8862: F, t352: F, t8712: F, t262: F, t7192: F, t7335: F, t7345: F, t2185: F, t9221: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t38552 = t7491 * t8355;
    let t38554 = t7491 * t8349;
    let t38556 = t35704 * t2416;
    let t38559 = t638 * t2160 * t8858;
    let t38562 = t638 * t2160 * t8862;
    let t38568 = t8712 * t352;
    let t38569 = t262 * t38568;
    let t38570 = t7192 * t38569;
    let t38608 = t7335 * t8355;
    let t38610 = t7345 * t8355;
    let t38621 = t9221 * t2185;
    (t38552, t38554, t38556, t38559, t38562, t38568, t38569, t38570, t38608, t38610, t38621)
}
