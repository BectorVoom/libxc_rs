//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2188/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2188(t90500: f64, t22716: f64, t7697: f64, t16452: f64, t26224: f64, t26225: f64, t80647: f64, t80659: f64, t80665: f64, t80667: f64, t80683: f64, t90460: f64, t90462: f64, t90466: f64, t90469: f64, t90471: f64, t90473: f64, t90477: f64, t90485: f64, t90491: f64, t90493: f64, t90496: f64, t90498: f64) -> f64 {
    let t90501 = 0.76763589786250567036e-1_f64 * t90500;
    let t90503 = t22716 * t7697;
    let t90505 = t90460 + 0.3289868133696452873e-1_f64 * t90462 + 0.16449340668482264365e-1_f64 * t90466 + t90469 + t90471 - t90473 + 0.3289868133696452873e-1_f64 * t90477 - 12.0_f64 * t26224 * t26225 * t16452 + 0.82246703342411321824e-2_f64 * t80647 - 0.49348022005446793095e-1_f64 * t90485 + 0.82246703342411321824e-2_f64 * t80659 - 0.3289868133696452873e-1_f64 * t90491 - t90493 + 0.76763589786250567036e-1_f64 * t80665 + 0.38381794893125283518e-1_f64 * t80667 - t90496 - 0.2302907693587517011e0_f64 * t90498 - t90501 - 0.24674011002723396547e-1_f64 * t80683 + 0.63969658155208805863e-1_f64 * t90503;
    t90505
}
