//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 880/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk880<F: Float>(t9482: F, t9492: F, t703: F, t3586: F, t713: F, t1108: F, t1911: F, t1977: F, t2829: F, t2849: F, t3578: F, t3581: F, t3608: F, t5825: F, t5835: F, t695: F, t714: F, t723: F, t7478: F, t9336: F, t9338: F, t9452: F, t9455: F, t9463: F, t9465: F) -> (F, F, F, F) {
    let t9493 = t9482 + t9492;
    let t9494 = t9493 * t703;
    let t9499 = t3586 * t713;
    let t9506 = F::cast_from(0.17315859105681463759e2_f64) * t1977 * t9452 + F::cast_from(0.34631718211362927518e2_f64) * t1977 * t9455 - F::cast_from(0.19751673498613801407e-1_f64) * t9463 + F::cast_from(0.5848223622634646207e0_f64) * t714 * t9465 + F::cast_from(0.17315859105681463759e2_f64) * t5835 * t3608 + F::new(1.0) * t1911 * t3578 + F::new(1.0) * t695 * t9494 + F::cast_from(0.32163958997385070134e2_f64) * t5825 * t3581 + F::cast_from(0.5848223622634646207e0_f64) * t9499 * t723 + F::cast_from(0.11696447245269292414e1_f64) * t7478 * t1108 + F::cast_from(0.11696447245269292414e1_f64) * t2829 * t2849 - t9336 - t9338;
    (t9493, t9494, t9499, t9506)
}
