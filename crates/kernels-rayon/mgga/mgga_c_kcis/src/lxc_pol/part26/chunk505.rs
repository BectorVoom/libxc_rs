//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 505/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk505(t174: f64, t4879: f64, t7: f64, t118: f64, t251: f64, t66: f64, t148: f64, t41: f64, t85: f64, t4532: f64, t447: f64, t2002: f64, t3734: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t175 = t174 <= zeta_threshold;
    let t4880 = t7 * t4879;
    let t4881 = t118 * t4880;
    let t4992 = t66 * t251;
    let t5133 = t85 * t148 * t41;
    let t5406 = piecewise3(t175, 0.0_f64, -t4532);
    let t5407 = t447 * t5406;
    let t5417 = t3734 * t2002;
    (t4880, t4881, t4992, t5133, t5406, t5407, t5417)
}
