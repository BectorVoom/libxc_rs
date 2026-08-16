//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2512/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2512(t3450: f64, t3475: f64, t426: f64, t43813: f64, t43816: f64, t3478: f64, t1179: f64, t12378: f64, t3488: f64, t3520: f64, t1175: f64, t12552: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t45085 = t426 / t3475 / t3450;
    let t45106 = 0.5356037037037037037e1_f64 * t43813;
    let t45107 = 0.16979925925925925926e1_f64 * t43816;
    let t45155 = t3475 * t3475;
    let t45157 = t426 / t45155;
    let t45158 = t3478 * t3478;
    let t45159 = 1.0_f64 / t45158;
    let t45163 = t12378 * t1179;
    let t45168 = t3488 * t3520;
    let t45174 = t1175 * t12552;
    (t45085, t45106, t45107, t45157, t45159, t45163, t45168, t45174)
}
