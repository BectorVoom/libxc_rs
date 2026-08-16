//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1123/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1123(t1336: f64, t22707: f64, t24099: f64, t26379: f64, t26381: f64, t26386: f64, t26390: f64, t26398: f64, t26412: f64, t26416: f64, t26419: f64, t26424: f64, t26427: f64, t27075: f64, t27078: f64, t27082: f64, t27086: f64, t27088: f64, t3777: f64, t5234: f64, t5334: f64, t5344: f64, t7209: f64, t7932: f64) -> f64 {
    let t27095 = 0.3289868133696452873e-1_f64 * t26379 + 0.76763589786250567037e-1_f64 * t26381 + 2.0_f64 * t5334 * t27075 - t24099 - t5344 * t27078 - 0.3289868133696452873e-1_f64 * t26386 - 0.3289868133696452873e-1_f64 * t26390 + t27082 - 0.3289868133696452873e-1_f64 * t26398 - t5234 * t7209 - t3777 * t7932 - t1336 * t27086 + t27088 + 0.82246703342411321825e-2_f64 * t22707 - 0.16449340668482264365e-1_f64 * t26412 + 0.3289868133696452873e-1_f64 * t26416 - 0.16449340668482264365e-1_f64 * t26419 + 0.3289868133696452873e-1_f64 * t26424 + 0.82246703342411321825e-2_f64 * t26427;
    t27095
}
