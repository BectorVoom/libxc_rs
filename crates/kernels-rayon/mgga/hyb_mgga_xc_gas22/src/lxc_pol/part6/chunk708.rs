//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 708/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk708(t3604: f64, t3605: f64, t1014: f64, t1016: f64, t1442: f64, t260: f64, t2609: f64, t3476: f64, t3479: f64, t3481: f64, t3484: f64, t3516: f64, t3520: f64, t3558: f64, t3587: f64, t3591: f64, t3597: f64, t3601: f64) -> (f64, f64) {
    let t3606 = t3604 * t3605;
    let t3609 = -t3476 + t3479 + t3481 - t3484 + t3516 + t3520 + t260 * t3587 + 0.19751673498613801407e-1_f64 * t260 * t3558 - 0.5848223622634646207e0_f64 * t3591 * t1016 - 0.5848223622634646207e0_f64 * t2609 * t1442 + 0.11696447245269292414e1_f64 * t1014 * t3597 - 0.5848223622634646207e0_f64 * t1014 * t3601 - 0.17315859105681463759e2_f64 * t1014 * t3606;
    (t3606, t3609)
}
