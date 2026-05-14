//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 478/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk478<F: Float>(t1452: F, t743: F, t1451: F, t3805: F, t1430: F, t3797: F, t1431: F, t733: F, t542: F, t1438: F, t738: F, t113: F, t3754: F, t3801: F, t1437: F, t104: F, t111: F, t120: F, t2642: F, t4023: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4073 = t743 * t1452;
    let t4075 = t1451 * t3805;
    let t4078 = t1430 * t3797;
    let t4081 = t733 * t1431;
    let t4083 = t1430 * t3805;
    let t4086 = t542 * t3797;
    let t4089 = t738 * t1438;
    let t4093 = t113 * t3754;
    let t4096 = t1430 * t3801;
    let t4099 = t1437 * t3801;
    let t4102 = t1451 * t3801;
    let t4105 = -0.23526125e-4 * t4073 + 0.50413125e-5 * t120 * t4075 - 0.672175e-5 * t120 * t4078 + 0.9368e-2 * t4081 - 0.3513e-2 * t104 * t4083 + 0.1171e-2 * t104 * t4086 - 0.26416666666666666666e-2 * t4089 - 0.23911438650126355246e-1 * t4023 * t2642 + 0.15538616723388920628e-3 * t4093 * t2642 + 0.7026e-2 * t104 * t4096 - 0.1585e-2 * t111 * t4099 - 0.10082625e-4 * t120 * t4102;
    (t4073, t4075, t4078, t4081, t4083, t4086, t4089, t4093, t4096, t4099, t4102, t4105)
}
