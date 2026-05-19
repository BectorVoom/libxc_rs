//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1069/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1069<F: Float>(t33273: F, t7967: F, t961: F, t11853: F, t2578: F, t7199: F, t33245: F, t33248: F, t33252: F, t33254: F, t33259: F, t33261: F, t33263: F, t33265: F, t33270: F) -> F {
    let t33275 = t7967 * t33273 * t961;
    let t33278 = t2578 * t7199 * t11853;
    let t33280 = -F::cast_from(0.1374296967252737644e-5_f64) * t33245 + F::cast_from(0.31675337336021900772e-5_f64) * t33248 + F::cast_from(0.31675337336021900772e-5_f64) * t33252 - F::cast_from(0.34752370105806885418e-3_f64) * t33254 - F::cast_from(0.86035622914632161465e-8_f64) * t33259 + F::cast_from(0.48908967355072681182e-6_f64) * t33261 + F::cast_from(0.10860115658064651693e-4_f64) * t33263 + F::cast_from(0.21720231316129303386e-4_f64) * t33265 + F::cast_from(0.4423264264475966605e-6_f64) * t33270 + F::cast_from(0.50589159825786619273e-8_f64) * t33275 + F::cast_from(0.12501199801949976838e-2_f64) * t33278;
    t33280
}
