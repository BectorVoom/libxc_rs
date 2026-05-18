//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1265/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1265<F: Float>(t3448: F, t4999: F, t14695: F, t26896: F, t1813: F, t9539: F, t3355: F, t13322: F, t3444: F, t5048: F, t92544: F, t14874: F, t283: F) -> (F, F, F, F, F, F, F) {
    let t95404 = t4999 * t3448;
    let t95406 = t26896 * t14695;
    let t95408 = t9539 * t1813;
    let t95410 = t4999 * t3355;
    let t95412 = t13322 * t3444;
    let t95414 = t92544 * t5048;
    let t95416 = t14874 * t283;
    (t95404, t95406, t95408, t95410, t95412, t95414, t95416)
}
