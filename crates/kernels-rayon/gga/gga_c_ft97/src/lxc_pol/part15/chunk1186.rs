//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1186/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1186(t5337: f64, t43548: f64, t91: f64, t5362: f64, t2755: f64, t193: f64, t22161: f64, t4056: f64, t89: f64, t4635: f64, t5408: f64, t13682: f64, t13688: f64, t15042: f64, t15047: f64, t15051: f64, t21973: f64, t2771: f64, t462: f64, t55201: f64, t70231: f64, t70799: f64, t70801: f64, t82769: f64, t82771: f64, t82773: f64, t83371: f64, t83373: f64, t89832: f64) -> (f64, f64, f64, f64) {
    let t90324 = t5337 * t5337;
    let t90326 = t91 * t43548 * t90324;
    let t90328 = t5362 * t5362;
    let t90330 = t91 * t2755 * t90328;
    let t90335 = t89 * t193 * t4056 * t22161;
    let t90337 = t5408 * t4635;
    let t90359 = 8.0_f64 / 3.0_f64 * t13682 * t15042 * t90337 - 8.0_f64 * t13688 * t15051 * t21973 - 8.0_f64 * t13688 * t15047 * t90337 + 8.0_f64 / 3.0_f64 * t82769 + 4.0_f64 / 9.0_f64 * t82771 + 40.0_f64 / 81.0_f64 * t82773 - 16.0_f64 / 27.0_f64 * t70231 + 112.0_f64 / 27.0_f64 * t55201 + 8.0_f64 * t83371 + 4.0_f64 / 3.0_f64 * t83373 - 8.0_f64 / 3.0_f64 * t70799 + 16.0_f64 / 3.0_f64 * t70801 + 2.0_f64 * t462 * t2771 * t89832;
    (t90326, t90330, t90335, t90359)
}
