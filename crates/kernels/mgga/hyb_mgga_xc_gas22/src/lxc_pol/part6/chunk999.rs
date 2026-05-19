//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 999/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk999<F: Float>(t1014: F, t1016: F, t1442: F, t2609: F, t2613: F, t2621: F, t3591: F, t3597: F, t3601: F, t3606: F, t7222: F, t9103: F, t9106: F, t9108: F, t9110: F, t9170: F, t9260: F, t9268: F, t9282: F, t9285: F, t9289: F, t9296: F, t9306: F) -> F {
    let t9309 = t9103 + t9106 + t9108 + t9110 + t9170 - t9260 + F::cast_from(0.11696447245269292414e1_f64) * t3591 * t2613 + F::cast_from(0.11696447245269292414e1_f64) * t1014 * t9282 - F::cast_from(0.17315859105681463759e2_f64) * t1014 * t9285 - F::cast_from(0.34631718211362927518e2_f64) * t1014 * t9289 + F::cast_from(0.23392894490538584828e1_f64) * t2609 * t3597 - F::cast_from(0.34631718211362927518e2_f64) * t2609 * t3606 - F::cast_from(0.11696447245269292414e1_f64) * t9296 * t1016 - F::cast_from(0.11696447245269292414e1_f64) * t2609 * t3601 - F::cast_from(0.17315859105681463759e2_f64) * t3591 * t2621 - F::cast_from(0.5848223622634646207e0_f64) * t7222 * t1442 - F::cast_from(0.5848223622634646207e0_f64) * t1014 * t9306 - t9268;
    t9309
}
