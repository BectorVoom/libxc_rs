//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1014/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1014(t4683: f64, t8082: f64, t3572: f64, t3642: f64, t2440: f64, t4706: f64, t3553: f64, t3610: f64, t3569: f64, t4744: f64, t72: f64, t732: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14034 = 12.0_f64 * t8082 * t4683;
    let t14035 = t3572 * t3642;
    let t14036 = 8.0_f64 * t14035;
    let t14040 = t2440 * t4706;
    let t14046 = t3553 * t3610;
    let t14050 = 8.0_f64 * t3572 * t3569;
    let t14051 = t4744 * t72;
    let t14052 = t14051 * t732;
    (t14034, t14036, t14040, t14046, t14050, t14052)
}
