//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1220/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1220<F: Float>(t22015: F, t25183: F, t22021: F, t2601: F, t10935: F, t23590: F, t23595: F, t23600: F, t25137: F, t25145: F, t25158: F, t25166: F, t25169: F, t25172: F, t25175: F, t25179: F, t25181: F, t2623: F, t322: F, t3835: F, t3836: F, t7427: F, t7430: F, t7457: F, t7461: F, t7464: F, t7478: F, t7488: F, t7921: F, t7926: F, t862: F, t867: F) -> (F, F, F) {
    let t25184 = t25183 * t22015;
    let t25188 = t2601 * t22021;
    let t25192 = F::cast_from(0.18933502127510156894e0_f64) * t25137 - F::cast_from(0.15146801702008125515e1_f64) * t7488 * t7461 + F::cast_from(0.2930329113747145654e3_f64) * t7430 * t7464 - F::cast_from(0.5860658227494291308e3_f64) * t7427 * t7457 - F::cast_from(0.20195735602677500687e1_f64) * t25145 - F::cast_from(0.15146801702008125515e1_f64) * t7488 * t7478 - F::cast_from(0.10866451862235947318e0_f64) * t3835 * t3836 * t23590 + F::cast_from(0.12073835402484385909e-1_f64) * t3835 * t3836 * t23595 + F::cast_from(0.48295341609937543636e-1_f64) * t3835 * t10935 * t23600 - F::new(154.0) / F::new(243.0) * t25158 * t867 - t2623 * t7921 / F::new(27.0) - F::new(28.0) / F::new(243.0) * t2623 * t7926 + t25166 / F::new(216.0) + F::new(7.0) / F::new(486.0) * t25169 + F::new(5.0) / F::new(972.0) * t25172 + t862 * t322 * t25175 / F::new(6.0) + F::new(11.0) / F::new(81.0) * t25179 + F::new(2.0) / F::new(81.0) * t25181 - t862 * t322 * t25184 / F::new(12.0) - t862 * t322 * t25188 / F::new(48.0);
    (t25184, t25188, t25192)
}
