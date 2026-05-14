//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1286/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1286<F: Float>(t2508: F, t2580: F, t32458: F, t32461: F, t32464: F, t32466: F, t32471: F, t32474: F, t32477: F, t32480: F, t32483: F, t32485: F, t32488: F, t32490: F, t39058: F, t39091: F, t7226: F) -> (F,) {
    let t39435 = 0.15381052460284448567e-1 * t2508 * t2580 * t39091 - 0.46143157380853345701e-1 * t2508 * t7226 * t39058 + t32458 - t32461 + t32464 + t32466 + t32471 + t32474 - t32477 + t32480 - t32483 - t32485 - t32488 - t32490;
    (t39435,)
}
