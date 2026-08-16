//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 490/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk490(t1430: f64, t3801: f64, t1437: f64, t1451: f64, t104: f64, t111: f64, t120: f64, t2642: f64, t4023: f64, t4073: f64, t4075: f64, t4078: f64, t4081: f64, t4083: f64, t4086: f64, t4089: f64, t4093: f64) -> (f64, f64, f64, f64) {
    let t4096 = t1430 * t3801;
    let t4099 = t1437 * t3801;
    let t4102 = t1451 * t3801;
    let t4105 = -0.23526125e-4_f64 * t4073 + 0.50413125e-5_f64 * t120 * t4075 - 0.672175e-5_f64 * t120 * t4078 + 0.9368e-2_f64 * t4081 - 0.3513e-2_f64 * t104 * t4083 + 0.1171e-2_f64 * t104 * t4086 - 0.26416666666666666666e-2_f64 * t4089 - 0.23911438650126355246e-1_f64 * t4023 * t2642 + 0.15538616723388920628e-3_f64 * t4093 * t2642 + 0.7026e-2_f64 * t104 * t4096 - 0.1585e-2_f64 * t111 * t4099 - 0.10082625e-4_f64 * t120 * t4102;
    (t4096, t4099, t4102, t4105)
}
