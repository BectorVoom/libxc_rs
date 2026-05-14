//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1053/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1053<F: Float>(t10647: F, t10650: F, t10654: F, t10657: F, t10661: F, t10662: F, t10668: F, t10672: F, t10679: F, t10705: F, t10720: F, t10771: F, t10776: F, t10801: F, t10841: F, t1379: F, t2322: F, t260: F, t3430: F, t3436: F, t3440: F, t4207: F, t4211: F, t4215: F, t856: F, t858: F, t8726: F) -> (F,) {
    let t10845 = -t10647 - t10650 + t10654 + t10657 + t10661 + 0.23392894490538584828e1 * t856 * t10662 - 0.17315859105681463759e2 * t2322 * t4215 + 0.10389515463408878255e3 * t856 * t10668 + 0.11696447245269292414e1 * t856 * t10672 - 0.11696447245269292414e1 * t8726 * t1379 + 0.11696447245269292414e1 * t2322 * t4207 - 0.5848223622634646207e0 * t10679 * t858 - 0.5848223622634646207e0 * t856 * t10705 - 0.5848223622634646207e0 * t2322 * t4211 + 0.23392894490538584828e1 * t3430 * t3436 - 0.11696447245269292414e1 * t3430 * t3440 - t10720 + t260 * (t10771 + t10776 + t10801 + t10841);
    (t10845,)
}
