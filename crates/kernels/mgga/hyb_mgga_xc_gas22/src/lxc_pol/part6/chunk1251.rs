//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1251/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1251<F: Float>(t10551: F, t839: F, t2311: F, t4175: F, t10838: F, t820: F, t2289: F, t3385: F, t2250: F, t4148: F, t10746: F, t10766: F, t1371: F, t21004: F, t2251: F, t2253: F, t2268: F, t2273: F, t2275: F, t2290: F, t2292: F, t2307: F, t2312: F, t2314: F, t2315: F, t24842: F, t28937: F, t29057: F, t29068: F, t29071: F, t3399: F, t3419: F, t4181: F, t4194: F, t6729: F, t829: F, t830: F, t848: F, t849: F, t8869: F, t8911: F) -> (F,) {
    let t29338 = t10551 * t839;
    let t29343 = t4175 * t2311;
    let t29352 = t10838 * t820;
    let t29361 = t4175 * t2289;
    let t29364 = t3385 * t3385;
    let t29371 = t4148 * t2250;
    let t29380 = 0.11696447245269292414e1 * t29338 * t849 + 0.5848223622634646207e0 * t10766 * t2307 + 0.17315859105681463759e2 * t29343 * t2315 + 0.23392894490538584828e1 * t8911 * t3419 + 0.11696447245269292414e1 * t3399 * t8869 - 0.11696447245269292414e1 * t21004 * t4181 + 2.0 * t29352 * t830 + 0.5848223622634646207e0 * t6729 * t4194 + t29057 - t29068 + 1.0 * t10746 * t2268 + 0.11696447245269292414e1 * t24842 * t1371 - t29071 - 0.11696447245269292414e1 * t29361 * t2292 + 0.64327917994770140268e2 * t2273 * t29364 * t2275 + 0.34631718211362927518e2 * t2312 * t28937 * t2314 - 2.0 * t29371 * t2253 - 4.0 * t2251 * t29364 * t829 - 0.23392894490538584828e1 * t2290 * t28937 * t848;
    (t29380,)
}
