//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 861/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk861<F: Float>(t1096: F, t13261: F, t1092: F, t1773: F, t3228: F, t1131: F, t3227: F, t4807: F, t9429: F, t2855: F, t4772: F, t2861: F, t4778: F) -> (F, F, F, F, F, F, F, F) {
    let t13262 = t1096 * t13261;
    let t13263 = t1092 * t13262;
    let t13265 = t1773 * t3228;
    let t13266 = t1131 * t13265;
    let t13267 = t3227 * t13266;
    let t13268 = t1092 * t13267;
    let t13270 = t9429 * t4807;
    let t13271 = F::cast_from(0.14739506172839506172e-2_f64) * t13270;
    let t13273 = t2855 * t4772;
    let t13274 = t1096 * t13273;
    let t13275 = t1092 * t13274;
    let t13277 = t2861 * t4778;
    (t13263, t13265, t13268, t13270, t13271, t13273, t13275, t13277)
}
