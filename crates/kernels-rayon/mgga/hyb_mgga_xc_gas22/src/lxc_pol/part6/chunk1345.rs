//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1345/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1345(t10551: f64, t839: f64, t2311: f64, t4175: f64, t10838: f64, t820: f64, t2289: f64, t3385: f64, t2250: f64, t4148: f64, t10746: f64, t10766: f64, t1371: f64, t21004: f64, t2251: f64, t2253: f64, t2268: f64, t2273: f64, t2275: f64, t2290: f64, t2292: f64, t2307: f64, t2312: f64, t2314: f64, t2315: f64, t24842: f64, t28937: f64, t29057: f64, t29068: f64, t29071: f64, t3399: f64, t3419: f64, t4181: f64, t4194: f64, t6729: f64, t829: f64, t830: f64, t848: f64, t849: f64, t8869: f64, t8911: f64) -> f64 {
    let t29338 = t10551 * t839;
    let t29343 = t4175 * t2311;
    let t29352 = t10838 * t820;
    let t29361 = t4175 * t2289;
    let t29364 = t3385 * t3385;
    let t29371 = t4148 * t2250;
    let t29380 = 0.11696447245269292414e1_f64 * t29338 * t849 + 0.5848223622634646207e0_f64 * t10766 * t2307 + 0.17315859105681463759e2_f64 * t29343 * t2315 + 0.23392894490538584828e1_f64 * t8911 * t3419 + 0.11696447245269292414e1_f64 * t3399 * t8869 - 0.11696447245269292414e1_f64 * t21004 * t4181 + 2.0_f64 * t29352 * t830 + 0.5848223622634646207e0_f64 * t6729 * t4194 + t29057 - t29068 + 1.0_f64 * t10746 * t2268 + 0.11696447245269292414e1_f64 * t24842 * t1371 - t29071 - 0.11696447245269292414e1_f64 * t29361 * t2292 + 0.64327917994770140268e2_f64 * t2273 * t29364 * t2275 + 0.34631718211362927518e2_f64 * t2312 * t28937 * t2314 - 2.0_f64 * t29371 * t2253 - 4.0_f64 * t2251 * t29364 * t829 - 0.23392894490538584828e1_f64 * t2290 * t28937 * t848;
    t29380
}
