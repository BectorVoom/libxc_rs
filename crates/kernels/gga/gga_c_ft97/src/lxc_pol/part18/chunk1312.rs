//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1312/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1312<F: Float>(t11437: F, t23909: F, t23671: F, t5899: F, t12945: F, t5900: F, t9432: F, t105343: F, t105347: F, t105351: F, t105355: F, t105359: F, t105362: F, t105366: F, t105370: F, t105374: F) -> (F, F, F, F) {
    let t105376 = t23909 * t11437;
    let t105378 = t5899 * t23671 * t105376;
    let t105381 = t5899 * t9432 * t5900 * t12945;
    let t105383 = 5.0 / 27.0 * t105343 - t105347 / 3.0 - 2.0 / 3.0 * t105351 + t105355 / 9.0 + t105359 - t105362 / 6.0 - 2.0 / 3.0 * t105366 - t105370 / 3.0 - 2.0 / 9.0 * t105374 + t105378 - 3.0 * t105381;
    (t105376, t105378, t105381, t105383)
}
