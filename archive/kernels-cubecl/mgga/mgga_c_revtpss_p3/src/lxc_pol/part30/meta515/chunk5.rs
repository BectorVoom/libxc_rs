//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1914/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1914<F: Float>(t1468: F, t1940: F, t1963: F, t2403: F, t25206: F, t25440: F, t27158: F, t27160: F, t27166: F, t27169: F, t27173: F, t27364: F, t27368: F, t27376: F, t27382: F, t27385: F, t27387: F, t27391: F, t27395: F, t27402: F, t27407: F, t30: F, t605: F, t7010: F, t7087: F, t7091: F, t7092: F, t7749: F, t7783: F, t7787: F) -> F {
    let t27408 = F::cast_from(3.0_f64) * t27158 * t27160 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t7087 * t7749 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t25206 * t27166 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t1963 * t27169 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t1963 * t27173 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t7783 * t7010 + t1940 * t27364 * t30 / F::cast_from(2.0_f64) - t1940 * t27368 * t7092 / F::cast_from(2.0_f64) + t1940 * t7783 * t605 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t25206 * t27376 - t1940 * t25440 * t7787 / F::cast_from(2.0_f64) + t27382 * t27385 - t1940 * t7091 * t27387 / F::cast_from(2.0_f64) - t1940 * t7091 * t27391 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t1963 * t27395 + t1940 * t7087 * t1468 / F::cast_from(2.0_f64) - t1940 * t7091 * t27402 / F::cast_from(2.0_f64) + t27407;
    t27408
}
