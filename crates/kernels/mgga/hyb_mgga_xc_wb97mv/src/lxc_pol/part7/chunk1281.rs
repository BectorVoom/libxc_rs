//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1281/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1281<F: Float>(t3435: F, t846: F, t10916: F, t10949: F, t11199: F, t11224: F, t1365: F, t2303: F, t2333: F, t2337: F, t2341: F, t260: F, t26553: F, t26745: F, t30795: F, t30946: F, t31002: F, t31052: F, t31103: F, t31152: F, t31205: F, t31254: F, t31372: F, t31406: F, t3447: F, t3460: F, t4264: F, t6960: F, t7034: F, t838: F, t847: F, t855: F, t9231: F) -> (F, F) {
    let t31413 = t3435 * t846;
    let t31438 = t260 * (t30946 + t31002 + t31052 + t31103 + t31152 + t31205 + t31254 + t31406) - 0.17315859105681463759e2 * t7034 * t4264 - 0.14035736694323150897e2 * t26553 * t1365 * t31413 + t30795 + 0.10389515463408878255e3 * t855 * t10916 * t6960 - 0.20508037716432813316e4 * t2333 * t10949 - 0.5848223622634646207e0 * t11224 * t2341 - 0.34631718211362927518e2 * t855 * t3460 * t26745 - 0.20508037716432813315e4 * t3447 * t9231 - 0.35089341735807877242e1 * t855 * t11199 * t2303 - 0.5848223622634646207e0 * t855 * t838 * t31372 * t847 + 0.11696447245269292414e1 * t11224 * t2337;
    (t31413, t31438)
}
