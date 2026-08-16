//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 848/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk848(t1092: f64, t13107: f64, t3317: f64, t5026: f64, t1022: f64, t330: f64, t1021: f64, t4994: f64, t1775: f64, t9528: f64, t1767: f64, t9476: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13108 = t1092 * t13107;
    let t13110 = t5026 * t3317;
    let t13111 = t1092 * t13110;
    let t13113 = t1022 * t330;
    let t13114 = t1021 * t13113;
    let t13115 = t4994 * t13114;
    let t13122 = t9528 * t1775;
    let t13124 = t9476 * t1767;
    (t13108, t13111, t13113, t13115, t13122, t13124)
}
