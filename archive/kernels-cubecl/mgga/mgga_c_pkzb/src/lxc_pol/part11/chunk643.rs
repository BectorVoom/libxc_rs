//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 643/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk643<F: Float>(t1096: F, t1108: F, t1916: F, t1938: F, t1955: F, t1977: F, t248: F, t2796: F, t2829: F, t3521: F, t3523: F, t3527: F, t3553: F, t3556: F, t3559: F, t3565: F, t3578: F, t3581: F, t3587: F, t3592: F, t3605: F, t3608: F, t695: F, t714: F) -> F {
    let t3611 = -F::cast_from(0.310907e-1_f64) * t3559 * t248 + F::cast_from(2.0_f64) * t2796 * t1096 - F::cast_from(2.0_f64) * t1916 * t3565 + F::cast_from(1.0_f64) * t695 * t3578 + F::cast_from(0.32163958997385070134e2_f64) * t1938 * t3581 + t3521 - t3523 + t3527 - t3553 - t3556 - F::cast_from(0.19751673498613801407e-1_f64) * t3587 + F::cast_from(0.11696447245269292414e1_f64) * t2829 * t1108 - F::cast_from(0.11696447245269292414e1_f64) * t1955 * t3592 + F::cast_from(0.5848223622634646207e0_f64) * t714 * t3605 + F::cast_from(0.17315859105681463759e2_f64) * t1977 * t3608;
    t3611
}
