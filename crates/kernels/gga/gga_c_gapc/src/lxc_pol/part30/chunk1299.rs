//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1299/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1299<F: Float>(t33209: F, t33212: F, t33214: F, t33217: F, t33221: F, t33226: F, t33228: F, t33230: F, t33232: F, t33240: F, t33242: F, t33245: F, t33248: F, t33252: F, t33254: F, t33259: F, t33261: F, t33263: F, t33265: F, t33270: F, t33275: F, t33278: F) -> (F, F) {
    let t37685 = F::new(0.27011279664738401692e-5) * t33209 + F::new(0.7246363367825880434e-6) * t33212 - F::new(0.40516919497107602538e-5) * t33214 + F::new(0.3623181683912940217e-6) * t33217 + F::new(0.33111854833537703651e-5) * t33221 + F::new(0.23968194627773771045e-6) * t33226 - F::new(0.5060221354166666667e-5) * t33228 + F::new(0.37101380443268507824e-3) * t33230 + F::new(0.30917817036057089854e-5) * t33232 - F::new(0.48751922435761895589e-4) * t33240 - F::new(0.13259130899812740005e-6) * t33242;
    let t37697 = -F::new(0.5497187869010950576e-5) * t33245 + F::new(0.12670134934408760308e-4) * t33248 + F::new(0.12670134934408760308e-4) * t33252 - F::new(0.13900948042322754167e-2) * t33254 - F::new(0.34414249165852864587e-7) * t33259 + F::new(0.19563586942029072472e-5) * t33261 + F::new(0.43440462632258606772e-4) * t33263 + F::new(0.86880925264517213544e-4) * t33265 + F::new(0.1769305705790386642e-5) * t33270 + F::new(0.2023566393031464771e-7) * t33275 + F::new(0.50004799207799907351e-2) * t33278;
    (t37685, t37697)
}
