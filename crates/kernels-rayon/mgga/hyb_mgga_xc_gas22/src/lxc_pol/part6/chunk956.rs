//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 956/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk956(t1347: f64, t2189: f64, t2234: f64, t1379: f64, t2322: f64, t2326: f64, t2334: f64, t3430: f64, t3436: f64, t3440: f64, t6759: f64, t856: f64, t858: f64, t8601: f64, t8605: f64, t8608: f64, t8613: f64, t8616: f64, t8620: f64, t8623: f64, t8627: f64, t8711: f64, t8725: f64, t8726: f64) -> (f64, f64, f64) {
    let t8731 = t1347 * t2189;
    let t8733 = 6.0_f64 * t2234 * t8731;
    let t8734 = -0.10254018858216406658e4_f64 * t856 * t8601 - 0.34631718211362927518e2_f64 * t856 * t8605 - 0.17315859105681463759e2_f64 * t856 * t8608 - t8613 - t8616 + t8620 + t8623 + t8627 - 0.5848223622634646207e0_f64 * t6759 * t1379 + 0.11696447245269292414e1_f64 * t3430 * t2326 - 0.5848223622634646207e0_f64 * t856 * t8711 - 0.17315859105681463759e2_f64 * t3430 * t2334 + 0.23392894490538584828e1_f64 * t2322 * t3436 - t8725 - 0.11696447245269292414e1_f64 * t8726 * t858 - 0.11696447245269292414e1_f64 * t2322 * t3440 + t8733;
    (t8731, t8733, t8734)
}
