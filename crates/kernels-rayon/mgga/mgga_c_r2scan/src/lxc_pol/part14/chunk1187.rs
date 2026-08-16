//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1187/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1187(t3579: f64, t38688: f64, t10610: f64, t11199: f64, t11509: f64, t11475: f64, t3262: f64, t11020: f64, t12203: f64, t11325: f64, t11486: f64, t10622: f64, t12098: f64, t3275: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41162 = 45.0_f64 / 64.0_f64 * t3579 * t38688;
    let t41165 = 3.0_f64 * t10610 * t11199 * t11509;
    let t41168 = 3.0_f64 / 2.0_f64 * t3262 * t11199 * t11475;
    let t41170 = 5.0_f64 / 16.0_f64 * t11020 * t12203;
    let t41173 = 15.0_f64 / 8.0_f64 * t3262 * t11325 * t11486;
    let t41176 = 5.0_f64 / 16.0_f64 * t3275 * t12098 * t10622;
    (t41162, t41165, t41168, t41170, t41173, t41176)
}
