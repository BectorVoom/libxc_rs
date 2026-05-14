//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1073/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1073<F: Float>(t2508: F, t7226: F, t7291: F, t8483: F, t21446: F, t3009: F, t21783: F, t27837: F, t27840: F, t27844: F, t27848: F, t27853: F, t27856: F, t27858: F, t27860: F, t471: F) -> (F, F, F, F) {
    let t32281 = 0.92286314761706691402e-1 * t2508 * t7226 * t8483 * t7291;
    let t32285 = 0.92286314761706691402e-1 * t2508 * t7226 * t3009 * t21446;
    let t32289 = 0.46143157380853345701e-1 * t2508 * t7226 * t3009 * t21783;
    let t32300 = (189.0 / 512.0 * t27837 - 2499.0 / 16384.0 * t27840 + 1239.0 / 524288.0 * t27844 - 441.0 / 0.16777216e8 * t27848 + 147.0 / 0.16777216e8 * t27853 - 413.0 / 524288.0 * t27856 + 833.0 / 16384.0 * t27858 - 63.0 / 512.0 * t27860) * t471;
    (t32281, t32285, t32289, t32300)
}
