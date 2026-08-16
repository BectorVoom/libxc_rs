//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1054/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1054(t39451: f64, t2604: f64, t35106: f64, t35110: f64, t35114: f64, t35118: f64, t39440: f64, t39445: f64, t39449: f64, t39453: f64, t39455: f64, t39457: f64, t39461: f64, t39463: f64, t39465: f64, t39470: f64, t39474: f64, t9620: f64) -> f64 {
    let t42970 = 0.3193131120497015617e0_f64 * t39451;
    let t42985 = -0.47885174879960069324e-4_f64 * t39440 - 0.638468998399467591e-4_f64 * t39445 - 0.212822999466489197e-4_f64 * t39449 + t42970 + 0.5107751987195740728e-4_f64 * t39453 - 0.1064114997332445985e-4_f64 * t39455 - 0.212822999466489197e-4_f64 * t39457 + 0.23948483403727617128e0_f64 * t2604 * t9620 - 0.5107751987195740728e-4_f64 * t39461 + 0.5107751987195740728e-4_f64 * t39463 + 0.638468998399467591e-4_f64 * t39465 - 0.212822999466489197e-4_f64 * t39470 + 0.85129199786595678799e-5_f64 * t39474 - 0.30487649791575028312e-3_f64 * t35106 + 0.43368970657079495308e-4_f64 * t35110 - 0.60975299583150056624e-3_f64 * t35114 + 0.86737941314158990616e-4_f64 * t35118;
    t42985
}
