//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1295/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1295<F: Float>(t100522: F, t26692: F, t23833: F, t925: F, t93048: F, t23826: F, t2059: F, t22591: F, t26743: F, t1008: F, t397: F, t554: F, t1354: F, t3347: F, t3379: F, t58: F) -> (F, F, F, F, F, F, F) {
    let t104888 = t26692 * t100522;
    let t104897 = t93048 * t925 * t23833;
    let t104901 = t93048 * t925 * t23826;
    let t104912 = t22591 * t26743 * t2059;
    let t104915 = t397 * t1008;
    let t104917 = t22591 * t104915 * t554;
    let t104920 = t3347 * t1354;
    let t104923 = t58 * t3379;
    (t104888, t104897, t104901, t104912, t104917, t104920, t104923)
}
