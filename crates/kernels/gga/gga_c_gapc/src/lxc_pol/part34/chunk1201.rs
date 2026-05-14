//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1201/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1201<F: Float>(t35222: F, t35225: F, t35228: F, t35231: F, t35234: F, t35203: F, t37236: F, t37237: F, t37238: F, t37239: F, t37240: F, t35240: F, t35243: F, t35246: F, t35249: F, t35252: F) -> (F, F, F, F, F, F) {
    let t37241 = 0.15018333275585850553e-5 * t35222;
    let t37242 = 0.6070699179094394313e-6 * t35225;
    let t37243 = 0.43440462632258606772e-4 * t35228;
    let t37244 = 0.11372686522837130914e-5 * t35231;
    let t37245 = 0.10567613244746075633e-6 * t35234;
    let t37246 = 0.7246363367825880434e-6 * t35203 + t37236 + t37237 - t37238 - t37239 + t37240 - t37241 + t37242 + t37243 + t37244 - t37245;
    let t37249 = 0.40483072916666666669e-4 * t35240;
    let t37250 = 0.2698871527777777778e-4 * t35243;
    let t37251 = 0.2698871527777777778e-4 * t35246;
    let t37252 = 0.17149079499421296297e-4 * t35249;
    let t37253 = 0.2748593934505475288e-6 * t35252;
    (t37246, t37249, t37250, t37251, t37252, t37253)
}
