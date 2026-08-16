//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 880/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk880(t9482: f64, t9492: f64, t703: f64, t3586: f64, t713: f64, t1108: f64, t1911: f64, t1977: f64, t2829: f64, t2849: f64, t3578: f64, t3581: f64, t3608: f64, t5825: f64, t5835: f64, t695: f64, t714: f64, t723: f64, t7478: f64, t9336: f64, t9338: f64, t9452: f64, t9455: f64, t9463: f64, t9465: f64) -> (f64, f64, f64, f64) {
    let t9493 = t9482 + t9492;
    let t9494 = t9493 * t703;
    let t9499 = t3586 * t713;
    let t9506 = 0.17315859105681463759e2_f64 * t1977 * t9452 + 0.34631718211362927518e2_f64 * t1977 * t9455 - 0.19751673498613801407e-1_f64 * t9463 + 0.5848223622634646207e0_f64 * t714 * t9465 + 0.17315859105681463759e2_f64 * t5835 * t3608 + 1.0_f64 * t1911 * t3578 + 1.0_f64 * t695 * t9494 + 0.32163958997385070134e2_f64 * t5825 * t3581 + 0.5848223622634646207e0_f64 * t9499 * t723 + 0.11696447245269292414e1_f64 * t7478 * t1108 + 0.11696447245269292414e1_f64 * t2829 * t2849 - t9336 - t9338;
    (t9493, t9494, t9499, t9506)
}
