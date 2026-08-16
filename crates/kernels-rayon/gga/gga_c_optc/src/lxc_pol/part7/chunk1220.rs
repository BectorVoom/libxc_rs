//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1220/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1220(t22015: f64, t25183: f64, t22021: f64, t2601: f64, t10935: f64, t23590: f64, t23595: f64, t23600: f64, t25137: f64, t25145: f64, t25158: f64, t25166: f64, t25169: f64, t25172: f64, t25175: f64, t25179: f64, t25181: f64, t2623: f64, t322: f64, t3835: f64, t3836: f64, t7427: f64, t7430: f64, t7457: f64, t7461: f64, t7464: f64, t7478: f64, t7488: f64, t7921: f64, t7926: f64, t862: f64, t867: f64) -> (f64, f64, f64) {
    let t25184 = t25183 * t22015;
    let t25188 = t2601 * t22021;
    let t25192 = 0.18933502127510156894e0_f64 * t25137 - 0.15146801702008125515e1_f64 * t7488 * t7461 + 0.2930329113747145654e3_f64 * t7430 * t7464 - 0.5860658227494291308e3_f64 * t7427 * t7457 - 0.20195735602677500687e1_f64 * t25145 - 0.15146801702008125515e1_f64 * t7488 * t7478 - 0.10866451862235947318e0_f64 * t3835 * t3836 * t23590 + 0.12073835402484385909e-1_f64 * t3835 * t3836 * t23595 + 0.48295341609937543636e-1_f64 * t3835 * t10935 * t23600 - 154.0_f64 / 243.0_f64 * t25158 * t867 - t2623 * t7921 / 27.0_f64 - 28.0_f64 / 243.0_f64 * t2623 * t7926 + t25166 / 216.0_f64 + 7.0_f64 / 486.0_f64 * t25169 + 5.0_f64 / 972.0_f64 * t25172 + t862 * t322 * t25175 / 6.0_f64 + 11.0_f64 / 81.0_f64 * t25179 + 2.0_f64 / 81.0_f64 * t25181 - t862 * t322 * t25184 / 12.0_f64 - t862 * t322 * t25188 / 48.0_f64;
    (t25184, t25188, t25192)
}
