//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1104/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1104(t10647: f64, t10650: f64, t10654: f64, t10657: f64, t10661: f64, t10662: f64, t10668: f64, t10672: f64, t10679: f64, t10705: f64, t10720: f64, t10771: f64, t10776: f64, t10801: f64, t10841: f64, t1379: f64, t2322: f64, t260: f64, t3430: f64, t3436: f64, t3440: f64, t4207: f64, t4211: f64, t4215: f64, t856: f64, t858: f64, t8726: f64) -> f64 {
    let t10845 = -t10647 - t10650 + t10654 + t10657 + t10661 + 0.23392894490538584828e1_f64 * t856 * t10662 - 0.17315859105681463759e2_f64 * t2322 * t4215 + 0.10389515463408878255e3_f64 * t856 * t10668 + 0.11696447245269292414e1_f64 * t856 * t10672 - 0.11696447245269292414e1_f64 * t8726 * t1379 + 0.11696447245269292414e1_f64 * t2322 * t4207 - 0.5848223622634646207e0_f64 * t10679 * t858 - 0.5848223622634646207e0_f64 * t856 * t10705 - 0.5848223622634646207e0_f64 * t2322 * t4211 + 0.23392894490538584828e1_f64 * t3430 * t3436 - 0.11696447245269292414e1_f64 * t3430 * t3440 - t10720 + t260 * (t10771 + t10776 + t10801 + t10841);
    t10845
}
