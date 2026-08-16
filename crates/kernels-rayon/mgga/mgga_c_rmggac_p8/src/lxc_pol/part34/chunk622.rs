//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 622/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk622(t15570: f64, t118: f64, t15516: f64, t15176: f64, t14444: f64, t551: f64, t5148: f64, t558: f64, t5266: f64, t15096: f64, t15099: f64, t15107: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15571 = 0.14967802127329760705e-1_f64 * t15570;
    let t15573 = 0.39914139006212695214e-1_f64 * t118 * t15516;
    let t15574 = 0.44903406381989282115e-1_f64 * t15176;
    let t15579 = t14444 * t551;
    let t15581 = 0.11974241701863808564e0_f64 * t5148 * t15579;
    let t15582 = t14444 * t558;
    let t15584 = 0.11974241701863808564e0_f64 * t5266 * t15582;
    let t15585 = 0.49892673757765869017e-2_f64 * t15096;
    let t15586 = 0.14967802127329760705e-1_f64 * t15099;
    let t15589 = 0.31062809106223861416e-2_f64 * t15107;
    (t15571, t15573, t15574, t15581, t15584, t15585, t15586, t15589)
}
