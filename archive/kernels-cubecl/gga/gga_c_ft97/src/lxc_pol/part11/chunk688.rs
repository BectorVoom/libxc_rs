//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 688/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk688<F: Float>(t236: F, t2417: F, t3724: F, t2427: F, t677: F, t2428: F, t689: F, t680: F, t2395: F, t709: F, t2394: F, t2380: F, t2382: F) -> (F, F, F, F, F, F) {
    let t9530 = t3724 * t236 * t2417;
    let t9533 = t677 * t2427;
    let t9534 = t689 * t2428;
    let t9535 = t680 * t9534;
    let t9538 = t2395 * t709;
    let t9539 = t2394 * t9538;
    let t9542 = t2380 * t2382;
    (t9530, t9533, t9535, t9538, t9539, t9542)
}
