//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 946/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk946<F: Float>(t13217: F, t3210: F, t3200: F, t4806: F, t9438: F, t4548: F, t4556: F, t4554: F, t3233: F, t5026: F, t1092: F, t13202: F, t13205: F, t13208: F, t13211: F, t13214: F, t9522: F) -> (F, F, F, F, F, F) {
    let t13218 = t3210 * t13217;
    let t13219 = t3200 * t13218;
    let t13221 = t9438 * t4806;
    let t13222 = t3200 * t13221;
    let t13224 = t9438 * t4548;
    let t13225 = t3200 * t13224;
    let t13227 = t9438 * t4556;
    let t13228 = t4554 * t13227;
    let t13230 = t5026 * t3233;
    let t13231 = t1092 * t13230;
    let t13234 = 0.11054629629629629629e-1 * t13202 - 0.33163888888888888888e-2 * t13205 + 0.66327777777777777776e-2 * t13208 + 0.33163888888888888888e-2 * t13211 + 0.16581944444444444444e-2 * t13214 + 0.11054629629629629629e-2 * t13219 - 0.58958024691358024689e-2 * t13222 + 0.17687407407407407407e-1 * t13225 - 0.14739506172839506172e-1 * t13228 + 0.13265555555555555555e-1 * t13231 + 0.11054629629629629629e-2 * t9522;
    (t13219, t13222, t13225, t13228, t13231, t13234)
}
