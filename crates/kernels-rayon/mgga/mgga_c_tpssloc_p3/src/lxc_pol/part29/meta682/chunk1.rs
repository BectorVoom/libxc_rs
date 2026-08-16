//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2305/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2305(t11881: f64, t15000: f64, t1653: f64, t1716: f64, t24778: f64, t24795: f64, t24829: f64, t27406: f64, t27531: f64, t3243: f64, t4964: f64, t7283: f64, t7362: f64, t7373: f64, t7376: f64, t7389: f64, t8073: f64, t8082: f64, t85814: f64, t85947: f64, t86076: f64, t95192: f64, t95194: f64, t95197: f64, t95201: f64, t95213: f64) -> f64 {
    let t95224 = -0.27415567780803773942e-2_f64 * t7283 * t7362 * t85947 * t1653 + 0.73108180748810063843e-2_f64 * t27406 * t24795 - t95192 - 0.3289868133696452873e-1_f64 * t95194 * t95197 + 0.16449340668482264365e-1_f64 * t95194 * t95201 - 0.82246703342411321825e-2_f64 * t7373 * t85814 * t8073 + 0.36554090374405031923e-2_f64 * t86076 * t27531 * t7376 * t3243 + t95213 + 6.0_f64 * t11881 * t8082 * t15000 - 0.9747757433174675179e-2_f64 * t27406 * t24778 - 0.82246703342411321825e-2_f64 * t7283 * t1716 * t24829 + 2.0_f64 * t4964 * t7389;
    t95224
}
