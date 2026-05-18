//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1399/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1399<F: Float>(t10963: F, t260: F, t1006: F, t1014: F, t1016: F, t10979: F, t11005: F, t1442: F, t25276: F, t2609: F, t2613: F, t2617: F, t2621: F, t29913: F, t30038: F, t30040: F, t3591: F, t3601: F, t4337: F, t4341: F, t7222: F, t9296: F, t9306: F, t997: F) -> F {
    let t30315 = t260 * t10963;
    let t30330 = t30038 + t30040 - F::new(0.5848223622634646207e0) * t1014 * t997 * t29913 * t1006 + F::new(0.11696447245269292414e1) * t10979 * t2613 + F::new(0.11696447245269292414e1) * t7222 * t4337 - F::new(0.5848223622634646207e0) * t10979 * t2617 - F::new(0.11696447245269292414e1) * t30315 * t1016 - F::new(0.11696447245269292414e1) * t25276 * t1442 - F::new(0.17315859105681463759e2) * t10979 * t2621 - F::new(0.23392894490538584828e1) * t9296 * t3601 - F::new(0.11696447245269292414e1) * t3591 * t9306 - F::new(0.5848223622634646207e0) * t7222 * t4341 - F::new(0.11696447245269292414e1) * t2609 * t11005;
    t30330
}
