//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2675/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2675(t2782: f64, t4086: f64, t49205: f64, t543: f64, t40270: f64, t5737: f64, t14127: f64, t14193: f64, t22016: f64, t4056: f64, t49177: f64, t49178: f64, t49187: f64, t49190: f64, t49199: f64, t49200: f64, t49203: f64, t5735: f64, t5745: f64, t9840: f64) -> f64 {
    let t49208 = t2782 * t4086 * t49205 * t543;
    let t49210 = t40270 * t5737;
    let t49212 = -t49177 + 0.17073386770573548589e-1_f64 * t49178 + t49187 - t49190 - 0.11853808529283920877e2_f64 * t14193 * t5735 * t22016 * t4056 + 0.39512695097613069591e1_f64 * t5745 * t14127 * t9840 + t49199 - 0.29272321618148349057e-1_f64 * t49200 - 0.30356481678079769392e-1_f64 * t49203 + 0.32927245914677557992e-1_f64 * t49208 - 0.2601984143835408805e-2_f64 * t49210;
    t49212
}
