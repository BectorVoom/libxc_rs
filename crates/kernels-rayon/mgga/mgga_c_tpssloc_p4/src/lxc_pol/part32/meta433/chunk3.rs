//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1669/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1669(t3726: f64, t6358: f64, t213: f64, t6347: f64, t1307: f64, t221: f64, t12228: f64, t12236: f64, t16078: f64, t16083: f64, t16099: f64, t16106: f64, t16108: f64, t16113: f64, t16119: f64, t5195: f64) -> f64 {
    let t19791 = t3726 * t6358;
    let t19793 = t213 * t6347;
    let t19795 = t221 * t19793 * t1307;
    let t19803 = 0.38888888888888888887e-2_f64 * t19791 + 0.49999999999999999998e-2_f64 * t5195 * t19795 + 0.16666666666666666666e-2_f64 * t12228 - 0.25925925925925925925e-1_f64 * t16078 - t16083 - t16099 - t12236 + 0.77777777777777777775e-2_f64 * t16106 - 0.10555555555555555555e-1_f64 * t16108 + t16113 + 0.33333333333333333332e-2_f64 * t16119;
    t19803
}
