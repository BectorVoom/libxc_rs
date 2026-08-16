//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1033/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1033(t22515: f64, t525: f64, t10195: f64, t10345: f64, t22508: f64, t22510: f64, t22513: f64, t22516: f64, t22522: f64, t22524: f64, t22526: f64, t515: f64, t534: f64) -> (f64, f64, f64) {
    let t22528 = t525 * t22515;
    let t22531 = -0.28769444444444444444e1_f64 * t22508 + 0.27618666666666666667e2_f64 * t22510 - 0.10229135802469135803e2_f64 * t22513 + 0.89504938271604938273e1_f64 * t22516 + 0.31310740740740740741e1_f64 * t10195 + 0.366775e-1_f64 * t22522 - 0.58684e0_f64 * t22524 + 0.65204444444444444445e0_f64 * t22526 + 0.5705388888888888889e0_f64 * t22528 + 0.13490888888888888889e1_f64 * t10345;
    let t22562 = 1.0_f64 * t515 * (-0.21099166666666666667e1_f64 * t22508 + 0.202552e2_f64 * t22510 - 0.75019259259259259258e1_f64 * t22513 + 0.6564185185185185185e1_f64 * t22516 + 0.31003950617283950618e1_f64 * t10195 + 0.68258333333333333335e-1_f64 * t22522 - 0.10921333333333333333e1_f64 * t22524 + 0.12134814814814814815e1_f64 * t22526 + 0.10617962962962962963e1_f64 * t22528 + 0.13388493827160493828e1_f64 * t10345) * t534;
    (t22528, t22531, t22562)
}
