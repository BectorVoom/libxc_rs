//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 566/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk566(t14194: f64, t14200: f64, t14202: f64, t3219: f64, t7720: f64, t498: f64, t698: f64, t515: f64, t7231: f64, t3351: f64, t8235: f64, t3352: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14661 = 0.2553875993597870364e-4_f64 * t14194;
    let t14662 = 0.1702583995731913576e-4_f64 * t14200;
    let t14663 = 0.85129199786595678799e-5_f64 * t14202;
    let t14664 = t7720 * t3219;
    let t14665 = 0.42564599893297839398e-5_f64 * t14664;
    let t14666 = t698 * t498;
    let t14667 = t515 * t14666;
    let t14668 = t7231 * t14667;
    let t14669 = t3351 * t14668;
    let t14670 = 0.42564599893297839398e-5_f64 * t14669;
    let t14671 = t515 * t8235;
    let t14672 = t3352 * t14671;
    (t14661, t14662, t14663, t14665, t14668, t14670, t14672)
}
