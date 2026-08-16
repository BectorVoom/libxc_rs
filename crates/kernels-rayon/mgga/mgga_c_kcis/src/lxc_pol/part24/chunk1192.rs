//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1192/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1192(t1014: f64, t27925: f64, t27882: f64, t26748: f64, t27803: f64, t27903: f64, t44544: f64, t7703: f64, t95890: f64, t1094: f64, t4923: f64, t27859: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t96123 = t1014 * t27925;
    let t96124 = 0.33163888888888888888e-2_f64 * t96123;
    let t96137 = t1014 * t27882;
    let t96138 = 0.33163888888888888888e-2_f64 * t96137;
    let t96148 = 0.15445601851851851852e-3_f64 * t26748 * t27803;
    let t96150 = t7703 * t44544 * t27903;
    let t96173 = t7703 * t95890;
    let t96210 = t4923 * t1094;
    let t96217 = t1014 * t27859;
    (t96123, t96124, t96137, t96138, t96148, t96150, t96173, t96210, t96217)
}
