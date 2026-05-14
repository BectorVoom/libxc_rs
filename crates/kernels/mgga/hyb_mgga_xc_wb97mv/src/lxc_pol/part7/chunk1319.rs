//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1319/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1319<F: Float>(t11395: F, t11424: F, t11437: F, t11443: F, t11446: F, t11447: F, t11453: F, t11501: F, t1428: F, t23366: F, t23400: F, t2535: F, t2573: F, t2574: F, t2589: F, t2595: F, t31960: F, t31962: F, t31965: F, t4360: F, t4373: F, t4376: F, t7254: F, t7259: F, t7316: F, t7333: F, t7409: F, t9385: F, t9458: F, t9544: F, t994: F) -> (F,) {
    let t32246 = 0.35089341735807877242e1 * t2595 * t4373 * t2574 - 0.14035736694323150897e2 * t7259 * t4360 * t2574 + 0.70178683471615754484e1 * t9458 * t9544 - 0.19298375398431042081e3 * t7333 * t11501 * t2535 - 0.24828486201251232145e5 * t23400 * t11424 * t2535 + t31960 - t31962 - t31965 - 0.12304822629859687989e5 * t23366 * t11453 * t2574 - 0.23392894490538584828e1 * t7409 * t11443 - 0.23392894490538584828e1 * t2573 * t11395 * t994 - 0.11696447245269292414e1 * t2573 * t4373 * t2589 - 0.10389515463408878255e3 * t7259 * t11446 * t2574 + 0.34631718211362927518e2 * t7254 * t11447 + 0.35089341735807877242e1 * t2595 * t4360 * t2589 + 0.6233709278045326953e3 * t7316 * t4376 * t2574 - 0.46785788981077169656e1 * t7409 * t11437 - 0.23392894490538584828e1 * t2573 * t1428 * t9385;
    (t32246,)
}
