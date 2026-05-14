//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 955/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk955<F: Float>(t1365: F, t2302: F, t3436: F, t846: F, t2317: F, t3439: F, t2325: F, t3435: F, t1364: F, t6968: F, t3403: F, t827: F, t1353: F, t2278: F, t2262: F, t2301: F, t2323: F, t3388: F, t3407: F, t3421: F, t3440: F, t6871: F, t6923: F, t6929: F, t6966: F, t6977: F, t6982: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9135 = t1365 * t2302;
    let t9142 = t3436 * t846;
    let t9145 = t1365 * t2317;
    let t9148 = t3439 * t2302;
    let t9151 = t3435 * t2325;
    let t9152 = t9151 * t846;
    let t9155 = t3439 * t2317;
    let t9158 = t1364 * t6968;
    let t9159 = t9158 * t2302;
    let t9166 = t3403 * t827;
    let t9169 = t1353 * t2278;
    let t9172 = 0.35089341735807877242e1 * t2323 * t9135 - 0.23392894490538584828e1 * t6929 * t3421 + 0.34631718211362927518e2 * t6977 * t3440 - 0.23392894490538584828e1 * t2301 * t9142 - 0.11696447245269292414e1 * t2301 * t9145 - 0.10389515463408878255e3 * t6982 * t9148 + 0.34631718211362927518e2 * t2323 * t9152 + 0.17315859105681463759e2 * t2323 * t9155 + 0.10254018858216406658e4 * t6966 * t9159 - 4.0 * t6923 * t3388 + 0.64327917994770140268e2 * t6871 * t3407 - 4.0 * t2262 * t9166 - 2.0 * t2262 * t9169;
    (t9135, t9142, t9145, t9148, t9151, t9152, t9155, t9158, t9159, t9166, t9169, t9172)
}
