//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1068/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1068<F: Float>(t33273: F, t7967: F, t961: F, t11853: F, t2578: F, t7199: F, t33245: F, t33248: F, t33252: F, t33254: F, t33259: F, t33261: F, t33263: F, t33265: F, t33270: F) -> F {
    let t33275 = t7967 * t33273 * t961;
    let t33278 = t2578 * t7199 * t11853;
    let t33280 = -F::new(0.1374296967252737644e-5) * t33245 + F::new(0.31675337336021900772e-5) * t33248 + F::new(0.31675337336021900772e-5) * t33252 - F::new(0.34752370105806885418e-3) * t33254 - F::new(0.86035622914632161465e-8) * t33259 + F::new(0.48908967355072681182e-6) * t33261 + F::new(0.10860115658064651693e-4) * t33263 + F::new(0.21720231316129303386e-4) * t33265 + F::new(0.4423264264475966605e-6) * t33270 + F::new(0.50589159825786619273e-8) * t33275 + F::new(0.12501199801949976838e-2) * t33278;
    t33280
}
