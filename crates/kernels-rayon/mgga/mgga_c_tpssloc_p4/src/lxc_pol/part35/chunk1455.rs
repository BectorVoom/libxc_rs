//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1455/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1455(t103218: f64, t103226: f64, t103332: f64, t104556: f64, t109697: f64, t1409: f64, t1761: f64, t2123: f64, t2144: f64, t2155: f64, t22008: f64, t22034: f64, t22113: f64, t24589: f64, t24601: f64, t27406: f64, t27792: f64, t29532: f64, t29551: f64, t29694: f64, t29809: f64, t466: f64, t4945: f64, t498: f64, t6243: f64, t6268: f64, t7283: f64, t7351: f64, t73613: f64, t73900: f64, t8002: f64, t8003: f64, t94332: f64, t94395: f64) -> f64 {
    let t109778 = 12.0_f64 * t4945 * t29532 - 0.80418998823691070229e-1_f64 * t103218 * t8003 - 0.54831135561607547883e-2_f64 * t103332 - 3.0_f64 * t27792 * t6268 - t73900 * t2155 + t22113 * t2144 * t498 - t73613 * t2155 + t466 * t109697 * t498 + 0.82246703342411321826e-2_f64 * t24589 * t103226 * t8002 - 0.43864908449286038307e-1_f64 * t94395 * t29809 - 0.16449340668482264365e-1_f64 * t24589 * t24601 * t94332 * t1409 * t6243 + 0.65797362673929057459e-1_f64 * t27406 * t29551 - 6.0_f64 * t104556 * t1761 - 0.82246703342411321825e-2_f64 * t7283 * t22034 * t2123 - 6.0_f64 * t7351 * t22008 + 0.43864908449286038307e-1_f64 * t27406 * t29694;
    t109778
}
