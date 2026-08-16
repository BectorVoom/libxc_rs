//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1072/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1072(t33273: f64, t7967: f64, t961: f64, t11853: f64, t2578: f64, t7199: f64, t33245: f64, t33248: f64, t33252: f64, t33254: f64, t33259: f64, t33261: f64, t33263: f64, t33265: f64, t33270: f64) -> f64 {
    let t33275 = t7967 * t33273 * t961;
    let t33278 = t2578 * t7199 * t11853;
    let t33280 = -0.1374296967252737644e-5_f64 * t33245 + 0.31675337336021900772e-5_f64 * t33248 + 0.31675337336021900772e-5_f64 * t33252 - 0.34752370105806885418e-3_f64 * t33254 - 0.86035622914632161465e-8_f64 * t33259 + 0.48908967355072681182e-6_f64 * t33261 + 0.10860115658064651693e-4_f64 * t33263 + 0.21720231316129303386e-4_f64 * t33265 + 0.4423264264475966605e-6_f64 * t33270 + 0.50589159825786619273e-8_f64 * t33275 + 0.12501199801949976838e-2_f64 * t33278;
    t33280
}
