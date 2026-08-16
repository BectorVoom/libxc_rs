//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1169/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1169(t13299: f64, t31115: f64, t40116: f64, t1788: f64, t31110: f64, t2041: f64, t5632: f64, t31495: f64, t31499: f64, t31505: f64, t31509: f64, t35673: f64, t35679: f64, t35683: f64, t35686: f64, t35703: f64, t35710: f64, t40105: f64, t40107: f64, t40109: f64, t40111: f64, t40114: f64) -> f64 {
    let t40118 = t31115 * t13299 * t40116;
    let t40121 = t31110 * t1788;
    let t40123 = t2041 * t5632;
    let t40125 = -t35673 + 0.17149607247227894789e-1_f64 * t40105 + t35679 - 0.17149607247227894789e-1_f64 * t40107 - 0.17149607247227894789e-2_f64 * t40109 - 0.17149607247227894789e-2_f64 * t40111 - t35683 - t35686 + 0.42874018118069736972e-3_f64 * t40114 + t35703 + 0.15724046144802076034e-2_f64 * t40118 - t31495 - t31499 - t35710 - 0.90035438047946447642e-2_f64 * t31505 - t31509 - 7.0_f64 / 48.0_f64 * t40121 - t40123 / 48.0_f64;
    t40125
}
