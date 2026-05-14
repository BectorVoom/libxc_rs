//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1139/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1139<F: Float>(t23177: F, t22513: F, t341: F, t346: F, t351: F, t343: F, t2515: F, t345: F, t2518: F, t2466: F, t2516: F, t7402: F, t933: F, t2472: F, t23171: F, t2473: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t23192 = 280.0 / 81.0 * t23177;
    let t23205 = 1.0 / t346 / t22513 / t351 / t341 / 96.0;
    let t23217 = f64::powf(t343, -0.25e1);
    let t23233 = t2515 * t2515;
    let t23235 = t345 / t23233;
    let t23237 = t2518 * t2518;
    let t23238 = 1.0 / t23237;
    let t23246 = t2466 * t2516;
    let t23254 = 0.96141975308641975307e-1 * t23177;
    let t23263 = t933 * t7402;
    let t23268 = t345 / t2515 / t2472;
    let t23279 = 0.13388493827160493828e1 * t23171;
    let t23281 = 0.31003950617283950618e1 * t23177;
    let t23302 = t2466 * t2473;
    (t23192, t23205, t23217, t23235, t23238, t23246, t23254, t23263, t23268, t23279, t23281, t23302)
}
