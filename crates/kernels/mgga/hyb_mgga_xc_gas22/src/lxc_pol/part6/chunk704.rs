//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 704/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk704<F: Float>(t1005: F, t3583: F, t1007: F, t1422: F, t1434: F, t2533: F, t2538: F, t2560: F, t2572: F, t2577: F, t2599: F, t3476: F, t3479: F, t3481: F, t3484: F, t3516: F, t3520: F, t3524: F, t3527: F, t3532: F, t3547: F, t3551: F, t3558: F, t3560: F, t3565: F, t3580: F, t374: F, t979: F, t988: F, t998: F) -> (F, F) {
    let t3584 = t3583 * t1005;
    let t3587 = -F::new(0.310907e-1) * t3524 * t374 + F::new(1.0) * t3527 * t988 + F::new(1.0) * t2533 * t1422 - F::new(2.0) * t2538 * t3532 + F::new(1.0) * t979 * t3547 + F::new(0.32163958997385070134e2) * t2560 * t3551 + t3476 - t3479 - t3481 + t3484 - t3516 - t3520 - F::new(0.19751673498613801407e-1) * t3558 + F::new(0.5848223622634646207e0) * t3560 * t1007 + F::new(0.5848223622634646207e0) * t2572 * t1434 - F::new(0.11696447245269292414e1) * t2577 * t3565 + F::new(0.5848223622634646207e0) * t998 * t3580 + F::new(0.17315859105681463759e2) * t2599 * t3584;
    (t3584, t3587)
}
