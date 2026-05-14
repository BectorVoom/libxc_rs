//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1072/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1072<F: Float>(t2322: F, t4242: F, t3461: F, t3460: F, t9151: F, t3436: F, t3452: F, t2300: F, t848: F, t4263: F, t260: F, t4224: F, t11096: F, t838: F, t847: F, t11040: F, t11043: F, t11118: F, t11129: F, t11165: F, t11194: F, t11195: F, t1373: F, t2333: F, t3447: F, t3453: F, t3457: F, t3462: F, t4256: F, t4260: F, t4264: F, t855: F, t857: F, t8934: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11199 = t2322 * t4242;
    let t11200 = t11199 * t3461;
    let t11203 = t3460 * t9151;
    let t11206 = t3452 * t3436;
    let t11211 = t2300 * t4242;
    let t11212 = t11211 * t848;
    let t11221 = t4263 * t848;
    let t11224 = t260 * t4224;
    let t11232 = t838 * t11096 * t847;
    let t11235 = t11040 + t11043 - 0.34631718211362927517e2 * t3447 * t3462 + t260 * (t11118 + t11129 + t11165 + t11195) - 0.17315859105681463759e2 * t855 * t11200 - 0.34631718211362927518e2 * t855 * t11203 + 0.23392894490538584828e1 * t855 * t11206 + 0.11696447245269292414e1 * t2333 * t4256 + 0.11696447245269292414e1 * t855 * t11212 - t11194 - 0.11696447245269292414e1 * t3447 * t3457 - 0.5848223622634646207e0 * t2333 * t4260 + 0.23392894490538584828e1 * t3447 * t3453 - 0.35089341735807877242e1 * t855 * t11221 - 0.5848223622634646207e0 * t11224 * t857 - 0.11696447245269292414e1 * t8934 * t1373 - 0.17315859105681463759e2 * t2333 * t4264 - 0.5848223622634646207e0 * t855 * t11232;
    (t11199, t11200, t11203, t11206, t11211, t11212, t11221, t11224, t11232, t11235)
}
