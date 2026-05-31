//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 632/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk632<F: Float>(t2882: F, t5413: F, t2881: F, t319: F, t4969: F, t835: F, t1901: F, t193: F, t2816: F, t4156: F, t4271: F, t4273: F, t4283: F, t446: F, t5311: F, t5315: F, t5319: F, t5323: F, t5327: F, t5332: F, t5376: F, t5381: F, t5395: F, t5399: F, t5403: F, t5410: F, t89: F) -> (F, F, F, F) {
    let t5414 = t2882 * t5413;
    let t5415 = t2881 * t5414;
    let t5419 = t835 * t319 * t4969;
    let t5422 = t2816 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t5311 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t446 * t5315 - t446 * t5319 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t446 * t5323 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t5327 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t5332 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4156 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4273 + t89 * t193 * t5376 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t5381 - t446 * t5395 / F::cast_from(3.0_f64) - t446 * t5399 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t5403 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4271 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t4283 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t5410 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t5415 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t446 * t5419;
    (t5414, t5415, t5419, t5422)
}
