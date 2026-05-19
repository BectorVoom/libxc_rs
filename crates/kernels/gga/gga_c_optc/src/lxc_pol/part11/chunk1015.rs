//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1015/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1015<F: Float>(t22515: F, t525: F, t10195: F, t10345: F, t22508: F, t22510: F, t22513: F, t22516: F, t22522: F, t22524: F, t22526: F, t515: F, t534: F) -> (F, F, F) {
    let t22528 = t525 * t22515;
    let t22531 = -F::cast_from(0.28769444444444444444e1_f64) * t22508 + F::cast_from(0.27618666666666666667e2_f64) * t22510 - F::cast_from(0.10229135802469135803e2_f64) * t22513 + F::cast_from(0.89504938271604938273e1_f64) * t22516 + F::cast_from(0.31310740740740740741e1_f64) * t10195 + F::new(0.366775e-1) * t22522 - F::new(0.58684e0) * t22524 + F::cast_from(0.65204444444444444445e0_f64) * t22526 + F::cast_from(0.5705388888888888889e0_f64) * t22528 + F::cast_from(0.13490888888888888889e1_f64) * t10345;
    let t22562 = F::new(1.0) * t515 * (-F::cast_from(0.21099166666666666667e1_f64) * t22508 + F::new(0.202552e2) * t22510 - F::cast_from(0.75019259259259259258e1_f64) * t22513 + F::cast_from(0.6564185185185185185e1_f64) * t22516 + F::cast_from(0.31003950617283950618e1_f64) * t10195 + F::cast_from(0.68258333333333333335e-1_f64) * t22522 - F::cast_from(0.10921333333333333333e1_f64) * t22524 + F::cast_from(0.12134814814814814815e1_f64) * t22526 + F::cast_from(0.10617962962962962963e1_f64) * t22528 + F::cast_from(0.13388493827160493828e1_f64) * t10345) * t534;
    (t22528, t22531, t22562)
}
