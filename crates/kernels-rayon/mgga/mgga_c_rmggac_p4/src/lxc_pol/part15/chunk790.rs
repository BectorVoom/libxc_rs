//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 790/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk790(t7491: f64, t8355: f64, t8349: f64, t2416: f64, t35704: f64, t2160: f64, t638: f64, t8858: f64, t8862: f64, t352: f64, t8712: f64, t262: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38552 = t7491 * t8355;
    let t38554 = t7491 * t8349;
    let t38556 = t35704 * t2416;
    let t38559 = t638 * t2160 * t8858;
    let t38560 = 0.81300399444200075504e-3_f64 * t38559;
    let t38562 = t638 * t2160 * t8862;
    let t38563 = 0.81300399444200075504e-3_f64 * t38562;
    let t38568 = t8712 * t352;
    let t38569 = t262 * t38568;
    (t38552, t38554, t38556, t38560, t38563, t38568, t38569)
}
