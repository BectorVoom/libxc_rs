//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 861/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk861(t18171: f64, t5441: f64, t4439: f64, t12140: f64, t617: f64, t5427: f64, t12217: f64, t16905: f64, t1928: f64, t610: f64, t990: f64, t4426: f64, t6141: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18172 = t18171 * t5441;
    let t18174 = t4439 * t18172 / 432.0_f64;
    let t18175 = t12140 * t617;
    let t18176 = t18175 * t5427;
    let t18178 = t4439 * t18176 / 648.0_f64;
    let t18183 = t12217 * t617;
    let t18187 = t16905 * t617;
    let t18192 = t610 * t1928 * t990;
    let t18205 = t6141 * t4426 / 324.0_f64;
    (t18174, t18175, t18178, t18183, t18187, t18192, t18205)
}
