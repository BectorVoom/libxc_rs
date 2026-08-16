//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1221/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1221(t1238: f64, t1252: f64, t2121: f64, t2155: f64, t3487: f64, t3593: f64, t498: f64, t7282: f64, t7283: f64, t7288: f64, t7291: f64, t7296: f64, t7303: f64, t7306: f64, t7349: f64, t7351: f64, t7356: f64, t7392: f64) -> f64 {
    let t7394 = t7282 - 0.27415567780803773942e-2_f64 * t7283 * t7288 - 0.82246703342411321825e-2_f64 * t7283 * t7291 + 0.82246703342411321825e-2_f64 * t2121 * t7296 - 0.82246703342411321825e-2_f64 * t7283 * t7303 + t7306 * t498 + t7349 * t498 - t7351 * t1252 - t3487 * t2155 - t3593 * t2155 + 2.0_f64 * t1238 * t7356 - t1238 * t7392;
    t7394
}
