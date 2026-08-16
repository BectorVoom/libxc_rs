//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1218/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1218(t10850: f64, t10864: f64, t1268: f64, t14487: f64, t21351: f64, t2265: f64, t231: f64, t2928: f64, t43109: f64, t43122: f64, t43194: f64, t43195: f64, t4342: f64, t4973: f64, t5457: f64, t54690: f64, t631: f64, t82342: f64, t82361: f64, t82367: f64, t82405: f64, t82407: f64, t82409: f64, t88239: f64, t88252: f64, t88606: f64, t88612: f64, t992: f64) -> f64 {
    let t91423 = -16.0_f64 * t2265 * t43109 * t82342 * t992 + 6.0_f64 * t2265 * t10864 * t4973 * t5457 - 16.0_f64 / 27.0_f64 * t2265 * t43122 * t21351 * t1268 + 4.0_f64 / 9.0_f64 * t2265 * t14487 * t88612 - 16.0_f64 * t82361 - 16.0_f64 / 3.0_f64 * t82367 + 6.0_f64 * t2265 * t4342 * t88606 + 8.0_f64 / 3.0_f64 * t82405 - 4.0_f64 / 9.0_f64 * t82407 + 8.0_f64 / 3.0_f64 * t82409 - 4.0_f64 * t631 * t231 * t10850 * t88252 - t631 * t231 * t2928 * t88239 + 14.0_f64 / 81.0_f64 * t631 * t43194 * t43195 * t88252 - 160.0_f64 / 27.0_f64 * t54690;
    t91423
}
