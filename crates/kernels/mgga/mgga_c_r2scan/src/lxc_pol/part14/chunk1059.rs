//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1059/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1059<F: Float>(t11199: F, t11550: F, t3262: F, t3275: F, t3472: F, t39339: F, t12210: F, t37513: F, t12197: F, t498: F, t3264: F, t3465: F, t40374: F, t7040: F, t3579: F, t38678: F) -> (F, F, F, F, F, F, F, F) {
    let t41196 = 3.0 / 2.0 * t3262 * t11199 * t11550;
    let t41199 = 5.0 / 8.0 * t3275 * t3472 * t39339;
    let t41201 = 3.0 / 4.0 * t37513 * t12210;
    let t41202 = t498 * t12197;
    let t41205 = 3.0 / 2.0 * t3262 * t41202 * t3264;
    let t41208 = 3.0 / 4.0 * t3262 * t3465 * t40374;
    let t41211 = t3275 * t11199 * t7040 / 2.0;
    let t41213 = t3579 * t38678 / 4.0;
    (t41196, t41199, t41201, t41202, t41205, t41208, t41211, t41213)
}
