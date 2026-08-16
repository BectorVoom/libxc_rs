//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 999/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk999(t1014: f64, t1016: f64, t1442: f64, t2609: f64, t2613: f64, t2621: f64, t3591: f64, t3597: f64, t3601: f64, t3606: f64, t7222: f64, t9103: f64, t9106: f64, t9108: f64, t9110: f64, t9170: f64, t9260: f64, t9268: f64, t9282: f64, t9285: f64, t9289: f64, t9296: f64, t9306: f64) -> f64 {
    let t9309 = t9103 + t9106 + t9108 + t9110 + t9170 - t9260 + 0.11696447245269292414e1_f64 * t3591 * t2613 + 0.11696447245269292414e1_f64 * t1014 * t9282 - 0.17315859105681463759e2_f64 * t1014 * t9285 - 0.34631718211362927518e2_f64 * t1014 * t9289 + 0.23392894490538584828e1_f64 * t2609 * t3597 - 0.34631718211362927518e2_f64 * t2609 * t3606 - 0.11696447245269292414e1_f64 * t9296 * t1016 - 0.11696447245269292414e1_f64 * t2609 * t3601 - 0.17315859105681463759e2_f64 * t3591 * t2621 - 0.5848223622634646207e0_f64 * t7222 * t1442 - 0.5848223622634646207e0_f64 * t1014 * t9306 - t9268;
    t9309
}
