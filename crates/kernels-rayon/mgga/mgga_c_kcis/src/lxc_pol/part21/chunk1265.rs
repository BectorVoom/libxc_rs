//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1265/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1265(t3448: f64, t4999: f64, t14695: f64, t26896: f64, t1813: f64, t9539: f64, t3355: f64, t13322: f64, t3444: f64, t5048: f64, t92544: f64, t14874: f64, t283: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95404 = t4999 * t3448;
    let t95406 = t26896 * t14695;
    let t95408 = t9539 * t1813;
    let t95410 = t4999 * t3355;
    let t95412 = t13322 * t3444;
    let t95414 = t92544 * t5048;
    let t95416 = t14874 * t283;
    (t95404, t95406, t95408, t95410, t95412, t95414, t95416)
}
