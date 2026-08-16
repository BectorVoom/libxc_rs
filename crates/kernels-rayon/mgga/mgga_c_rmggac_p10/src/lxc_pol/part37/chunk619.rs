//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 619/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk619(t14444: f64, t551: f64, t5148: f64, t558: f64, t5266: f64, t15096: f64, t15099: f64, t15107: f64, t15110: f64, t15112: f64, t15114: f64, t15120: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15579 = t14444 * t551;
    let t15581 = 0.11974241701863808564e0_f64 * t5148 * t15579;
    let t15582 = t14444 * t558;
    let t15584 = 0.11974241701863808564e0_f64 * t5266 * t15582;
    let t15585 = 0.49892673757765869017e-2_f64 * t15096;
    let t15586 = 0.14967802127329760705e-1_f64 * t15099;
    let t15589 = 0.31062809106223861416e-2_f64 * t15107;
    let t15590 = 0.5177134851037310236e-2_f64 * t15110;
    let t15591 = 0.66380770525302906696e-3_f64 * t15112;
    let t15592 = 0.99571155787954360044e-3_f64 * t15114;
    let t15595 = 0.14464861606874801909e-3_f64 * t15120;
    (t15581, t15584, t15585, t15586, t15589, t15590, t15591, t15592, t15595)
}
