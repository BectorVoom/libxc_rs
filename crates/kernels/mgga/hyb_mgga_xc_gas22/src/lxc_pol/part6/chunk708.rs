//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 708/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk708<F: Float>(t3604: F, t3605: F, t1014: F, t1016: F, t1442: F, t260: F, t2609: F, t3476: F, t3479: F, t3481: F, t3484: F, t3516: F, t3520: F, t3558: F, t3587: F, t3591: F, t3597: F, t3601: F) -> (F, F) {
    let t3606 = t3604 * t3605;
    let t3609 = -t3476 + t3479 + t3481 - t3484 + t3516 + t3520 + t260 * t3587 + F::cast_from(0.19751673498613801407e-1_f64) * t260 * t3558 - F::cast_from(0.5848223622634646207e0_f64) * t3591 * t1016 - F::cast_from(0.5848223622634646207e0_f64) * t2609 * t1442 + F::cast_from(0.11696447245269292414e1_f64) * t1014 * t3597 - F::cast_from(0.5848223622634646207e0_f64) * t1014 * t3601 - F::cast_from(0.17315859105681463759e2_f64) * t1014 * t3606;
    (t3606, t3609)
}
