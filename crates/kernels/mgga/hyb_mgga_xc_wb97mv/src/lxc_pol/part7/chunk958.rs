//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 958/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk958<F: Float>(t2300: F, t3435: F, t848: F, t1373: F, t2333: F, t2337: F, t2345: F, t260: F, t3447: F, t3453: F, t3457: F, t3462: F, t7034: F, t855: F, t857: F, t8902: F, t8933: F, t8934: F, t8937: F, t8941: F, t9002: F, t9074: F, t9114: F, t9117: F, t9119: F, t9133: F, t9172: F, t9214: F) -> (F, F) {
    let t9218 = t2300 * t3435;
    let t9219 = t9218 * t848;
    let t9224 = -0.11696447245269292414e1 * t2333 * t3457 - 0.17315859105681463759e2 * t3447 * t2345 - 0.5848223622634646207e0 * t7034 * t1373 - 0.34631718211362927518e2 * t855 * t8902 - t8933 - 0.11696447245269292414e1 * t8934 * t857 + 0.11696447245269292414e1 * t855 * t8937 + 0.10389515463408878255e3 * t855 * t8941 - 0.5848223622634646207e0 * t855 * t9002 - 0.34631718211362927518e2 * t2333 * t3462 + 0.23392894490538584828e1 * t2333 * t3453 + t260 * (t9074 + t9133 + t9172 + t9214) + 0.23392894490538584828e1 * t855 * t9219 + 0.11696447245269292414e1 * t3447 * t2337 + t9114 + t9117 + t9119;
    (t9219, t9224)
}
