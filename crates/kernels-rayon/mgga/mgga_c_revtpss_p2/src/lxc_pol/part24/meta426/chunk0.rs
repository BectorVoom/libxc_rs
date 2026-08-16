//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1376/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1376(t3475: f64, t426: f64, t3478: f64, t43752: f64, t439: f64, t3519: f64, t3522: f64, t43813: f64, t1209: f64, t13126: f64, t17708: f64, t44842: f64, t487: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t45155 = t3475 * t3475;
    let t45157 = t426 / t45155;
    let t45158 = t3478 * t3478;
    let t45159 = 1.0_f64 / t45158;
    let t45177 = t439 * t43752;
    let t45186 = t3519 * t3519;
    let t45187 = 1.0_f64 / t45186;
    let t45188 = t439 * t45187;
    let t45189 = t3522 * t3522;
    let t45190 = 1.0_f64 / t45189;
    let t45232 = 0.17757530864197530864e0_f64 * t43813;
    let t45371 = t1209 * t13126 * t17708;
    let t45438 = t44842 * t487;
    (t45157, t45159, t45177, t45187, t45188, t45190, t45232, t45371, t45438)
}
