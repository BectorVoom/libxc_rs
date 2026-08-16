//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1218/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1218<F: Float>(t10850: F, t10864: F, t1268: F, t14487: F, t21351: F, t2265: F, t231: F, t2928: F, t43109: F, t43122: F, t43194: F, t43195: F, t4342: F, t4973: F, t5457: F, t54690: F, t631: F, t82342: F, t82361: F, t82367: F, t82405: F, t82407: F, t82409: F, t88239: F, t88252: F, t88606: F, t88612: F, t992: F) -> F {
    let t91423 = -F::cast_from(16.0_f64) * t2265 * t43109 * t82342 * t992 + F::cast_from(6.0_f64) * t2265 * t10864 * t4973 * t5457 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t2265 * t43122 * t21351 * t1268 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2265 * t14487 * t88612 - F::cast_from(16.0_f64) * t82361 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t82367 + F::cast_from(6.0_f64) * t2265 * t4342 * t88606 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t82405 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t82407 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t82409 - F::cast_from(4.0_f64) * t631 * t231 * t10850 * t88252 - t631 * t231 * t2928 * t88239 + F::cast_from(14.0_f64) / F::cast_from(81.0_f64) * t631 * t43194 * t43195 * t88252 - F::cast_from(160.0_f64) / F::cast_from(27.0_f64) * t54690;
    t91423
}
