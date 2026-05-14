//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 948/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk948<F: Float>(t5042: F, t922: F, t3202: F, t3200: F, t1767: F, t3219: F, t3218: F, t1096: F, t1092: F, t1773: F, t3228: F, t1131: F, t3227: F, t4807: F, t9429: F, t2855: F, t4772: F) -> (F, F, F, F, F, F, F, F) {
    let t13256 = t5042 * t922;
    let t13257 = t3202 * t13256;
    let t13258 = t3200 * t13257;
    let t13260 = t1767 * t3219;
    let t13261 = t3218 * t13260;
    let t13262 = t1096 * t13261;
    let t13263 = t1092 * t13262;
    let t13265 = t1773 * t3228;
    let t13266 = t1131 * t13265;
    let t13267 = t3227 * t13266;
    let t13268 = t1092 * t13267;
    let t13270 = t9429 * t4807;
    let t13271 = 0.14739506172839506172e-2 * t13270;
    let t13273 = t2855 * t4772;
    (t13258, t13260, t13263, t13265, t13268, t13270, t13271, t13273)
}
