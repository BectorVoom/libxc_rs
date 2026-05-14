//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 432/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk432<F: Float>(t3066: F, t5540: F, t6: F, t78: F, t388: F, t58: F, sigma0: F) -> (F, F, F, F) {
    let t5541 = t5540 * t3066;
    let t5544 = t78 * t6;
    let t5545 = t388 * t5544;
    let t5546 = sigma0 * t58;
    (t5541, t5544, t5545, t5546)
}
