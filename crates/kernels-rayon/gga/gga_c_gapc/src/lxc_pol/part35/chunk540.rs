//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 540/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk540(t3121: f64, t3123: f64, t3077: f64, t3082: f64, t3086: f64, t3089: f64, t3092: f64, t3098: f64, t3101: f64, t3106: f64, t3110: f64, t3118: f64) -> f64 {
    let t3124 = t3121 * t3123;
    let t3126 = -0.50028749986204251383e-8_f64 * t3077 - 0.33735894097222222223e-5_f64 * t3082 - 0.10120768229166666667e-4_f64 * t3086 - 0.86880925264517213544e-4_f64 * t3089 - 0.86880925264517213544e-4_f64 * t3092 + 0.7240077105376434462e-6_f64 * t3098 - 0.11594181388521408695e-4_f64 * t3101 - 0.13900948042322754167e-2_f64 * t3106 - 0.13900948042322754167e-2_f64 * t3110 + 0.28137654660407340484e-8_f64 * t3118 + 0.28137654660407340485e-7_f64 * t3124;
    t3126
}
