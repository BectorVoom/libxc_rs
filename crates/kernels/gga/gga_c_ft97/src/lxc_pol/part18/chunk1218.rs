//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1218/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1218<F: Float>(t100409: F, t100372: F, t100376: F, t100380: F, t100384: F, t100388: F, t100392: F, t100395: F, t100398: F, t100402: F, t100406: F, t100413: F, t100427: F, t100430: F, t100419: F, t100423: F, t100434: F, t100438: F, t100443: F, t100447: F, t100451: F, t100457: F, t100462: F, t93350: F) -> (F, F) {
    let t102144 = t100409 / 6.0;
    let t102146 = -2.0 / 3.0 * t100372 + 5.0 / 27.0 * t100376 + t100380 - 4.0 / 3.0 * t100384 - 4.0 / 3.0 * t100388 + 4.0 / 9.0 * t100392 + 2.0 * t100395 + 2.0 * t100398 + 4.0 / 3.0 * t100402 + t100406 / 3.0 - t102144 - 8.0 / 9.0 * t100413;
    let t102150 = 2.0 / 9.0 * t100427;
    let t102151 = 2.0 / 3.0 * t100430;
    let t102159 = 8.0 / 3.0 * t100419 + 2.0 / 3.0 * t100423 + t93350 - t102150 + t102151 - 6.0 * t100434 + t100438 / 3.0 + 2.0 / 3.0 * t100443 + 24.0 * t100447 + t100451 / 3.0 + t100457 / 3.0 - t100462 / 9.0;
    (t102146, t102159)
}
