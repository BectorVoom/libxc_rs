//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1186/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1186<F: Float>(t5337: F, t43548: F, t91: F, t5362: F, t2755: F, t193: F, t22161: F, t4056: F, t89: F, t4635: F, t5408: F, t13682: F, t13688: F, t15042: F, t15047: F, t15051: F, t21973: F, t2771: F, t462: F, t55201: F, t70231: F, t70799: F, t70801: F, t82769: F, t82771: F, t82773: F, t83371: F, t83373: F, t89832: F) -> (F, F, F, F) {
    let t90324 = t5337 * t5337;
    let t90326 = t91 * t43548 * t90324;
    let t90328 = t5362 * t5362;
    let t90330 = t91 * t2755 * t90328;
    let t90335 = t89 * t193 * t4056 * t22161;
    let t90337 = t5408 * t4635;
    let t90359 = F::new(8.0) / F::new(3.0) * t13682 * t15042 * t90337 - F::new(8.0) * t13688 * t15051 * t21973 - F::new(8.0) * t13688 * t15047 * t90337 + F::new(8.0) / F::new(3.0) * t82769 + F::new(4.0) / F::new(9.0) * t82771 + F::new(40.0) / F::new(81.0) * t82773 - F::new(16.0) / F::new(27.0) * t70231 + F::new(112.0) / F::new(27.0) * t55201 + F::new(8.0) * t83371 + F::new(4.0) / F::new(3.0) * t83373 - F::new(8.0) / F::new(3.0) * t70799 + F::new(16.0) / F::new(3.0) * t70801 + F::new(2.0) * t462 * t2771 * t89832;
    (t90326, t90330, t90335, t90359)
}
