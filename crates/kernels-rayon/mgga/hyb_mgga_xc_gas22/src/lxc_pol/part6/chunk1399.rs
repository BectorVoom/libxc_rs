//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1399/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1399(t10963: f64, t260: f64, t1006: f64, t1014: f64, t1016: f64, t10979: f64, t11005: f64, t1442: f64, t25276: f64, t2609: f64, t2613: f64, t2617: f64, t2621: f64, t29913: f64, t30038: f64, t30040: f64, t3591: f64, t3601: f64, t4337: f64, t4341: f64, t7222: f64, t9296: f64, t9306: f64, t997: f64) -> f64 {
    let t30315 = t260 * t10963;
    let t30330 = t30038 + t30040 - 0.5848223622634646207e0_f64 * t1014 * t997 * t29913 * t1006 + 0.11696447245269292414e1_f64 * t10979 * t2613 + 0.11696447245269292414e1_f64 * t7222 * t4337 - 0.5848223622634646207e0_f64 * t10979 * t2617 - 0.11696447245269292414e1_f64 * t30315 * t1016 - 0.11696447245269292414e1_f64 * t25276 * t1442 - 0.17315859105681463759e2_f64 * t10979 * t2621 - 0.23392894490538584828e1_f64 * t9296 * t3601 - 0.11696447245269292414e1_f64 * t3591 * t9306 - 0.5848223622634646207e0_f64 * t7222 * t4341 - 0.11696447245269292414e1_f64 * t2609 * t11005;
    t30330
}
