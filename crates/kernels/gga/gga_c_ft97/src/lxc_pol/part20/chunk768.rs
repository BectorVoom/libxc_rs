//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 768/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk768<F: Float>(t193: F, t24494: F, t89: F, t24435: F, t24441: F, t24445: F, t24452: F, t24455: F, t24458: F, t24463: F, t24468: F, t24470: F, t24475: F, t24480: F, t24483: F, t24485: F, t24489: F, t24492: F) -> (F, F) {
    let t24495 = t193 * t24494;
    let t24496 = t89 * t24495;
    let t24498 = -2.0 / 3.0 * t24435 - t24441 / 6.0 - t24445 / 3.0 - 3.0 / 8.0 * t24452 - t24455 / 6.0 - 4.0 / 3.0 * t24458 + 2.0 * t24463 + t24468 - 2.0 / 3.0 * t24470 + t24475 / 2.0 - t24480 - t24483 + 2.0 / 3.0 * t24485 - t24489 + 4.0 * t24492 + 2.0 * t24496;
    (t24496, t24498)
}
