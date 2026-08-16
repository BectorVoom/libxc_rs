//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1316/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1316(t1092: f64, t26671: f64, t27792: f64, t26753: f64, t1094: f64, t4923: f64, t1122: f64, t303: f64, t1134: f64, t4924: f64, t1014: f64, t27859: f64) -> (f64, f64, f64, f64, f64) {
    let t96204 = t1092 * t26671 * t27792;
    let t96207 = t1092 * t26753 * t27792;
    let t96210 = t4923 * t1094;
    let t96212 = t303 * t96210 * t1122;
    let t96215 = t303 * t4924 * t1134;
    let t96217 = t1014 * t27859;
    (t96204, t96207, t96212, t96215, t96217)
}
