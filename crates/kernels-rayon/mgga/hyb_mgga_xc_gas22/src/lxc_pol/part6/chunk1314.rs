//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1314/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1314(t2188: f64, t2228: f64, t4140: f64, t10651: f64, t2189: f64, t6579: f64, t2234: f64, t10622: f64, t10679: f64, t1379: f64, t2322: f64, t2334: f64, t24788: f64, t28730: f64, t3430: f64, t3435: f64, t3440: f64, t856: f64, t8711: f64, t8726: f64, t8869: f64, t8934: f64, t8938: f64) -> (f64, f64, f64, f64) {
    let t28733 = 2.0_f64 * t2188 * t4140 * t2228;
    let t28736 = 0.96491876992155210402e2_f64 * t6579 * t10651 * t2189;
    let t28739 = 0.16081979498692535067e2_f64 * t2234 * t10651 * t2228;
    let t28740 = -0.23392894490538584828e1_f64 * t8726 * t3440 + 0.23392894490538584828e1_f64 * t856 * t3435 * t8869 - 0.11696447245269292414e1_f64 * t24788 * t1379 - 0.11696447245269292414e1_f64 * t3430 * t8711 - 0.17315859105681463759e2_f64 * t10679 * t2334 + 0.2077903092681775651e3_f64 * t3430 * t8938 - 0.69263436422725855036e2_f64 * t2322 * t10622 + 0.23392894490538584828e1_f64 * t3430 * t8934 - t28730 - t28733 - t28736 + t28739;
    (t28733, t28736, t28739, t28740)
}
