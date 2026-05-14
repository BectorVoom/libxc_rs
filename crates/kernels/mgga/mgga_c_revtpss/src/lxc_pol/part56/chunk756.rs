//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 756/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk756<F: Float>(t27383: F, t27384: F, t1583: F, t605: F, t30: F, t4537: F, t1468: F, t775: F, t890: F, t1940: F, t1963: F, t2255: F, t2403: F, t25206: F, t25440: F, t27158: F, t27160: F, t27166: F, t27169: F, t27173: F, t27364: F, t27368: F, t27376: F, t27382: F, t7010: F, t7087: F, t7091: F, t7092: F, t7749: F, t7783: F, t7787: F) -> (F, F, F, F, F, F, F) {
    let t27385 = t27383 * t27384;
    let t27387 = t605 * t1583;
    let t27391 = t30 * t4537;
    let t27395 = t1468 * t775;
    let t27402 = t1468 * t890;
    let t27407 = t1940 * t1963 * t2255;
    let t27408 = 3.0 * t27158 * t27160 + 3.0 / 2.0 * t2403 * t7087 * t7749 - 3.0 / 2.0 * t25206 * t27166 + 3.0 / 2.0 * t2403 * t1963 * t27169 + 3.0 / 2.0 * t2403 * t1963 * t27173 + 3.0 / 2.0 * t2403 * t7783 * t7010 + t1940 * t27364 * t30 / 2.0 - t1940 * t27368 * t7092 / 2.0 + t1940 * t7783 * t605 / 2.0 - 3.0 / 2.0 * t25206 * t27376 - t1940 * t25440 * t7787 / 2.0 + t27382 * t27385 - t1940 * t7091 * t27387 / 2.0 - t1940 * t7091 * t27391 / 2.0 + 3.0 / 2.0 * t2403 * t1963 * t27395 + t1940 * t7087 * t1468 / 2.0 - t1940 * t7091 * t27402 / 2.0 + t27407;
    (t27385, t27387, t27391, t27395, t27402, t27407, t27408)
}
