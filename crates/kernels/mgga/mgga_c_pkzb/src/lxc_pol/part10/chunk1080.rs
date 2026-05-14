//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1080/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1080<F: Float>(t1108: F, t1911: F, t1977: F, t2829: F, t2849: F, t3578: F, t3581: F, t3608: F, t5825: F, t5835: F, t695: F, t714: F, t723: F, t7478: F, t9336: F, t9338: F, t9452: F, t9455: F, t9463: F, t9465: F, t9494: F, t9499: F) -> (F,) {
    let t9506 = 0.17315859105681463759e2 * t1977 * t9452 + 0.34631718211362927518e2 * t1977 * t9455 - 0.19751673498613801407e-1 * t9463 + 0.5848223622634646207e0 * t714 * t9465 + 0.17315859105681463759e2 * t5835 * t3608 + 1.0 * t1911 * t3578 + 1.0 * t695 * t9494 + 0.32163958997385070134e2 * t5825 * t3581 + 0.5848223622634646207e0 * t9499 * t723 + 0.11696447245269292414e1 * t7478 * t1108 + 0.11696447245269292414e1 * t2829 * t2849 - t9336 - t9338;
    (t9506,)
}
