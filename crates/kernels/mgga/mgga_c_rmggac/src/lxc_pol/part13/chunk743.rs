//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 743/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk743<F: Float>(t7491: F, t8349: F, t2416: F, t35704: F, t2160: F, t638: F, t8858: F, t8862: F, t2347: F, t839: F, t262: F, t36629: F, t352: F, t8712: F, t7192: F, t16043: F, t9190: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t38554 = t7491 * t8349;
    let t38556 = t35704 * t2416;
    let t38559 = t638 * t2160 * t8858;
    let t38562 = t638 * t2160 * t8862;
    let t38564 = t2347 * t839;
    let t38565 = t262 * t38564;
    let t38566 = t36629 * t38565;
    let t38568 = t8712 * t352;
    let t38569 = t262 * t38568;
    let t38570 = t7192 * t38569;
    let t38572 = t16043 * t9190;
    (t38554, t38556, t38559, t38562, t38564, t38565, t38566, t38568, t38569, t38570, t38572)
}
