//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1754/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1754(t9902: f64, t2535: f64, t4199: f64, t1471: f64, t32: f64, t2659: f64, t9910: f64, t4095: f64, t67: f64, t758: f64, t9922: f64, t118: f64, t1474: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13112 = 0.18311447306006545054e-3_f64 * t9902;
    let t13113 = t4199 * t2535;
    let t13114 = 0.5848223622634646207e0_f64 * t13113;
    let t13115 = t32 * t1471;
    let t13117 = 12.0_f64 * t13115 * t2659;
    let t13118 = 4.0_f64 * t9910;
    let t13119 = t4095 * t67;
    let t13121 = 0.36622894612013090108e-3_f64 * t13119 * t758;
    let t13122 = 0.11696447245269292414e1_f64 * t9922;
    let t13123 = t1474 * t118;
    (t13112, t13113, t13114, t13115, t13117, t13118, t13119, t13121, t13122, t13123)
}
