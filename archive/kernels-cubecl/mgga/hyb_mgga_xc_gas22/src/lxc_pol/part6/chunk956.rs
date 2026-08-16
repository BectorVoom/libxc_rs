//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 956/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk956<F: Float>(t1347: F, t2189: F, t2234: F, t1379: F, t2322: F, t2326: F, t2334: F, t3430: F, t3436: F, t3440: F, t6759: F, t856: F, t858: F, t8601: F, t8605: F, t8608: F, t8613: F, t8616: F, t8620: F, t8623: F, t8627: F, t8711: F, t8725: F, t8726: F) -> (F, F, F) {
    let t8731 = t1347 * t2189;
    let t8733 = F::cast_from(6.0_f64) * t2234 * t8731;
    let t8734 = -F::cast_from(0.10254018858216406658e4_f64) * t856 * t8601 - F::cast_from(0.34631718211362927518e2_f64) * t856 * t8605 - F::cast_from(0.17315859105681463759e2_f64) * t856 * t8608 - t8613 - t8616 + t8620 + t8623 + t8627 - F::cast_from(0.5848223622634646207e0_f64) * t6759 * t1379 + F::cast_from(0.11696447245269292414e1_f64) * t3430 * t2326 - F::cast_from(0.5848223622634646207e0_f64) * t856 * t8711 - F::cast_from(0.17315859105681463759e2_f64) * t3430 * t2334 + F::cast_from(0.23392894490538584828e1_f64) * t2322 * t3436 - t8725 - F::cast_from(0.11696447245269292414e1_f64) * t8726 * t858 - F::cast_from(0.11696447245269292414e1_f64) * t2322 * t3440 + t8733;
    (t8731, t8733, t8734)
}
