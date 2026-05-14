//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1226/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1226<F: Float>(t10651: F, t2228: F, t2234: F, t10622: F, t10679: F, t1379: F, t2322: F, t2334: F, t24788: F, t28730: F, t28733: F, t28736: F, t3430: F, t3435: F, t3440: F, t856: F, t8711: F, t8726: F, t8869: F, t8934: F, t8938: F) -> (F, F) {
    let t28739 = 0.16081979498692535067e2 * t2234 * t10651 * t2228;
    let t28740 = -0.23392894490538584828e1 * t8726 * t3440 + 0.23392894490538584828e1 * t856 * t3435 * t8869 - 0.11696447245269292414e1 * t24788 * t1379 - 0.11696447245269292414e1 * t3430 * t8711 - 0.17315859105681463759e2 * t10679 * t2334 + 0.2077903092681775651e3 * t3430 * t8938 - 0.69263436422725855036e2 * t2322 * t10622 + 0.23392894490538584828e1 * t3430 * t8934 - t28730 - t28733 - t28736 + t28739;
    (t28739, t28740)
}
