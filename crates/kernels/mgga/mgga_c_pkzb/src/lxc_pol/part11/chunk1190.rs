//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1190/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1190<F: Float>(t31511: F, t881: F, t890: F, t898: F, t10150: F, t8170: F, t3833: F, t8028: F, t3136: F, t3840: F, t10164: F, t3147: F, t11213: F, t2295: F, t11319: F, t11322: F, t11326: F, t22503: F, t2328: F, t237: F, t31058: F, t31472: F, t31521: F, t31523: F, t31582: F, t31584: F, t31586: F, t3161: F, t891: F, t9985: F) -> (F, F, F, F, F, F) {
    let t31957 = 0.5848223622634646207e0 * t898 * t881 * t31511 * t890;
    let t31960 = 0.31168546390226634765e3 * t898 * t10150 * t8170;
    let t31962 = 0.35089341735807877242e1 * t8028 * t3833;
    let t31965 = 0.10526802520742363173e2 * t898 * t3840 * t3136;
    let t31967 = 0.10389515463408878255e3 * t3147 * t10164;
    let t31969 = t2295 * t11213;
    let t31986 = 0.11696447245269292414e1 * t898 * t31969 * t891 + 0.19751673498613801407e-1 * t237 * t31472 + 0.31168546390226634766e3 * t22503 * t9985 * t31058 - t31521 + t31523 + t31582 + t31584 + t31586 - 0.6233709278045326953e3 * t898 * t11322 * t3161 + 0.14035736694323150897e2 * t898 * t11326 * t891 - 0.51947577317044391277e2 * t2328 * t11319;
    (t31957, t31960, t31962, t31965, t31967, t31986)
}
