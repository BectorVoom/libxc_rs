//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1312/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1312(t1014: f64, t27882: f64, t1092: f64, t1804: f64, t26760: f64, t3316: f64, t26748: f64, t27803: f64, t27903: f64, t44544: f64, t7703: f64, t27763: f64, t3228: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96137 = t1014 * t27882;
    let t96138 = 0.33163888888888888888e-2_f64 * t96137;
    let t96141 = t1092 * t26760 * t1804 * t3316;
    let t96148 = 0.15445601851851851852e-3_f64 * t26748 * t27803;
    let t96150 = t7703 * t44544 * t27903;
    let t96154 = t1092 * t27763 * t1804 * t3228;
    (t96137, t96138, t96141, t96148, t96150, t96154)
}
