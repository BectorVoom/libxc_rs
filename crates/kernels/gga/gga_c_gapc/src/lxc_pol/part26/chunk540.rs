//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 540/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk540<F: Float>(t3121: F, t3123: F, t3077: F, t3082: F, t3086: F, t3089: F, t3092: F, t3098: F, t3101: F, t3106: F, t3110: F, t3118: F) -> F {
    let t3124 = t3121 * t3123;
    let t3126 = -F::cast_from(0.50028749986204251383e-8_f64) * t3077 - F::cast_from(0.33735894097222222223e-5_f64) * t3082 - F::cast_from(0.10120768229166666667e-4_f64) * t3086 - F::cast_from(0.86880925264517213544e-4_f64) * t3089 - F::cast_from(0.86880925264517213544e-4_f64) * t3092 + F::cast_from(0.7240077105376434462e-6_f64) * t3098 - F::cast_from(0.11594181388521408695e-4_f64) * t3101 - F::cast_from(0.13900948042322754167e-2_f64) * t3106 - F::cast_from(0.13900948042322754167e-2_f64) * t3110 + F::cast_from(0.28137654660407340484e-8_f64) * t3118 + F::cast_from(0.28137654660407340485e-7_f64) * t3124;
    t3126
}
