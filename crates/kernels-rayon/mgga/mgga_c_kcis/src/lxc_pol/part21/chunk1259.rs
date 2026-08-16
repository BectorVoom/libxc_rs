//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1259/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1259(t5073: f64, t92437: f64, t14700: f64, t7748: f64, t15082: f64, t26896: f64, t26929: f64, t5177: f64, t9531: f64, t380: f64, t5182: f64, t92514: f64) -> (f64, f64, f64, f64, f64) {
    let t95336 = t92437 * t5073;
    let t95338 = t7748 * t14700;
    let t95340 = t26896 * t15082;
    let t95343 = t9531 * t26929 * t5177;
    let t95346 = t380 * t92514 * t5182;
    (t95336, t95338, t95340, t95343, t95346)
}
