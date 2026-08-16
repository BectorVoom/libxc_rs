//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1128/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1128(t3127: f64, t381: f64, t23602: f64, t1011: f64, t1615: f64, t1014: f64, t23665: f64, t7611: f64, t1936: f64, t362: f64, t2775: f64, t1625: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25483 = t3127 * t381;
    let t25484 = t23602 * t25483;
    let t25485 = t1615 * t1011;
    let t25490 = t1014 * t381;
    let t25491 = t23602 * t25490;
    let t25508 = t23665 * t7611;
    let t25510 = t1936 * t362;
    let t25511 = t381 * t2775;
    let t25516 = t362 * t1625;
    (t25484, t25485, t25490, t25491, t25508, t25510, t25511, t25516)
}
