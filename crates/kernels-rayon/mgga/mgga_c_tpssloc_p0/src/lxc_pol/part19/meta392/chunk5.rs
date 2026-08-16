//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1486/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1486(t11931: f64, t225: f64, t11604: f64, t496: f64, t68: f64, t3599: f64, t11601: f64, t11599: f64, t11606: f64, t11608: f64, t11613: f64, t11868: f64, t1190: f64, t11919: f64, t11925: f64, t11928: f64, t11935: f64, t1238: f64, t1252: f64, t3487: f64, t3593: f64, t3600: f64, t3630: f64, t3631: f64, t45314: f64, t466: f64, t498: f64) -> f64 {
    let t45345 = t11931 * t225;
    let t45349 = 1.0_f64 / t11604 / t496;
    let t45350 = t68 * t45349;
    let t45351 = t3599 * t3599;
    let t45355 = t11601 * t225;
    let t45375 = t11599 * t225;
    let t45382 = -36.0_f64 * t11606 * t1238 * t3599 * t3630 + 4.0_f64 * t11868 * t1190 * t498 + 24.0_f64 * t1238 * t45350 * t45351 + t45314 * t466 * t498 - 24.0_f64 * t11608 * t3487 - 12.0_f64 * t11613 * t3631 - 4.0_f64 * t11919 * t3593 - 6.0_f64 * t11925 * t3631 + 12.0_f64 * t11928 * t3600 + 24.0_f64 * t11935 * t3487 - 12.0_f64 * t1252 * t45345 - 12.0_f64 * t1252 * t45355 - 4.0_f64 * t1252 * t45375;
    t45382
}
