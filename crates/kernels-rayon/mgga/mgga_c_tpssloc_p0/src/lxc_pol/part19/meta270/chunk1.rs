//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1027/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1027(t11599: f64, t11601: f64, t11608: f64, t11613: f64, t11919: f64, t11923: f64, t11925: f64, t11928: f64, t11931: f64, t11935: f64, t1238: f64, t1252: f64, t3487: f64, t3593: f64, t3600: f64, t3631: f64, t498: f64) -> f64 {
    let t11940 = t11599 * t498 + 3.0_f64 * t11601 * t498 - 6.0_f64 * t11608 * t1238 - 6.0_f64 * t11613 * t1252 - t11919 * t1238 + t11923 * t498 - 3.0_f64 * t11925 * t1252 - 3.0_f64 * t11928 * t1252 + 3.0_f64 * t11931 * t498 + 6.0_f64 * t11935 * t1238 + 6.0_f64 * t3487 * t3600 - 3.0_f64 * t3487 * t3631 + 6.0_f64 * t3593 * t3600 - 3.0_f64 * t3593 * t3631;
    t11940
}
