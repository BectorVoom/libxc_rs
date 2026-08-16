//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 704/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk704(t1005: f64, t3583: f64, t1007: f64, t1422: f64, t1434: f64, t2533: f64, t2538: f64, t2560: f64, t2572: f64, t2577: f64, t2599: f64, t3476: f64, t3479: f64, t3481: f64, t3484: f64, t3516: f64, t3520: f64, t3524: f64, t3527: f64, t3532: f64, t3547: f64, t3551: f64, t3558: f64, t3560: f64, t3565: f64, t3580: f64, t374: f64, t979: f64, t988: f64, t998: f64) -> (f64, f64) {
    let t3584 = t3583 * t1005;
    let t3587 = -0.310907e-1_f64 * t3524 * t374 + 1.0_f64 * t3527 * t988 + 1.0_f64 * t2533 * t1422 - 2.0_f64 * t2538 * t3532 + 1.0_f64 * t979 * t3547 + 0.32163958997385070134e2_f64 * t2560 * t3551 + t3476 - t3479 - t3481 + t3484 - t3516 - t3520 - 0.19751673498613801407e-1_f64 * t3558 + 0.5848223622634646207e0_f64 * t3560 * t1007 + 0.5848223622634646207e0_f64 * t2572 * t1434 - 0.11696447245269292414e1_f64 * t2577 * t3565 + 0.5848223622634646207e0_f64 * t998 * t3580 + 0.17315859105681463759e2_f64 * t2599 * t3584;
    (t3584, t3587)
}
