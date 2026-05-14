//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 576/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk576<F: Float>(t1901: F, t193: F, t2816: F, t4156: F, t4271: F, t4273: F, t4283: F, t446: F, t5311: F, t5315: F, t5319: F, t5323: F, t5327: F, t5332: F, t5376: F, t5381: F, t5395: F, t5399: F, t5403: F, t5410: F, t5415: F, t5419: F, t89: F) -> (F,) {
    let t5422 = t2816 + 2.0 / 3.0 * t446 * t5311 - 2.0 / 9.0 * t446 * t5315 - t446 * t5319 / 9.0 - 2.0 / 27.0 * t446 * t5323 + 2.0 / 3.0 * t446 * t5327 + 2.0 / 3.0 * t446 * t5332 + 2.0 / 9.0 * t4156 + 2.0 / 9.0 * t4273 + t89 * t193 * t5376 / 3.0 - 2.0 / 3.0 * t446 * t5381 - t446 * t5395 / 3.0 - t446 * t5399 / 3.0 - 2.0 / 3.0 * t446 * t5403 - 2.0 / 9.0 * t4271 + 2.0 / 27.0 * t4283 + 2.0 / 9.0 * t1901 * t5410 + 2.0 / 9.0 * t1901 * t5415 + 2.0 / 9.0 * t446 * t5419;
    (t5422,)
}
