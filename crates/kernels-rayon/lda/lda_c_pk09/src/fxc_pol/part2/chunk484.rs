//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 484/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk484(t2703: f64, t420: f64, t1181: f64, t1182: f64, t1183: f64, t1184: f64, t1702: f64, t1689: f64, t1692: f64, t253: f64) -> (f64, f64, f64, f64) {
    let t2704 = t2703 * t420;
    let t2707 = t1181 + t1182 + t1183 + t1184;
    let t2708 = t1702 * t2707;
    let t2711 = t1689 - t1692 + 1.28_f64 * t253 * t2704 - 1.28_f64 * t253 * t2708;
    (t2704, t2707, t2708, t2711)
}
