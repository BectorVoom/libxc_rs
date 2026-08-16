//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 946/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk946(t3127: f64, t381: f64, t23602: f64, t1011: f64, t1615: f64, t4594: f64, t1014: f64, t1023: f64, t1022: f64, t7593: f64, t1060: f64, t1945: f64, t4649: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25483 = t3127 * t381;
    let t25484 = t23602 * t25483;
    let t25485 = t1615 * t1011;
    let t25486 = t25485 * t4594;
    let t25487 = t25484 * t25486;
    let t25490 = t1014 * t381;
    let t25491 = t23602 * t25490;
    let t25492 = t25485 * t1023;
    let t25493 = t25491 * t25492;
    let t25496 = t7593 * t1022;
    let t25497 = t25496 * t1060;
    let t25499 = t1945 * t4649;
    (t25486, t25487, t25492, t25493, t25496, t25497, t25499)
}
