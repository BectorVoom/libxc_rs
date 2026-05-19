//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1201/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1201<F: Float>(t2787: F, t2773: F, t2790: F, t2783: F, t1037: F, t1075: F, t1081: F, t1090: F, t1096: F, t21841: F, t21846: F, t21864: F, t21866: F, t21869: F, t21872: F, t21875: F, t21880: F, t21882: F, t21884: F, t21886: F, t21889: F, t21951: F, t22141: F, t22151: F, t22154: F, t22181: F, t2636: F, t2639: F, t2772: F, t2789: F, t2791: F, t2802: F, t2809: F, t470: F, t483: F, t7241: F, t7269: F, t7316: F, t7323: F, t7398: F, t7469: F, t7477: F, t7516: F) -> (F, F) {
    let t22282 = t2787 * t2787;
    let t22285 = t2773 * t2773;
    let t22286 = t2790 * t2790;
    let t22316 = t2783 * t2783;
    let t22344 = F::cast_from(0.91082604192152556044e5_f64) * t483 * t22151 * t21846 * t22154 + F::cast_from(0.19964560303604640732e6_f64) * t470 / t22282 * t22285 / t22286 + F::new(1.0) * t1075 * (-F::cast_from(0.39219166666666666667e1_f64) * t21864 + F::new(0.376504e2) * t21866 - F::cast_from(0.13944592592592592593e2_f64) * t21869 + F::cast_from(0.12201518518518518519e2_f64) * t21872 + F::cast_from(0.5356037037037037037e1_f64) * t21875 + F::cast_from(0.14025833333333333333e0_f64) * t21880 - F::cast_from(0.22441333333333333332e1_f64) * t21882 + F::cast_from(0.24934814814814814815e1_f64) * t21884 + F::cast_from(0.21817962962962962963e1_f64) * t21886 + F::cast_from(0.16979925925925925926e1_f64) * t21889) * t1081 - F::cast_from(0.14035736694323150897e2_f64) * t7398 * t21846 * t1096 + F::cast_from(0.1301229756036208781e0_f64) * t1037 * t7269 - F::cast_from(0.19263893255070628431e1_f64) * t1037 * t7516 - F::cast_from(0.12304822629859687989e5_f64) * t483 * t22181 * t21846 * t7241 + F::cast_from(0.96491876992155210402e2_f64) * t2789 * t22316 * t2791 + t22141 - F::cast_from(0.62337092780453269531e3_f64) * t7398 * t7323 * t2636 - F::cast_from(0.35089341735807877242e1_f64) * t2802 * t21841 * t1096 + F::cast_from(0.51947577317044391277e2_f64) * t2809 * t21841 * t2639 + F::cast_from(0.5848223622634646207e0_f64) * t1090 * t21951 * t1096 + F::cast_from(0.11579025239058625248e4_f64) * t7469 * t22285 * t2791 + F::cast_from(0.6233709278045326953e3_f64) * t7316 * t21846 * t2639 - F::new(6.0) * t2772 * t22316 * t1081 - F::new(24.0) * t7477 * t22285 * t1081;
    (t22285, t22344)
}
