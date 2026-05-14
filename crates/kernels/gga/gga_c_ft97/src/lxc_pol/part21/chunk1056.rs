//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1056/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1056<F: Float>(t3056: F, t5537: F, t5546: F, t5569: F, t6441: F, t92652: F, t22582: F, t37835: F, t22535: F, t420: F, t8042: F, t92896: F, t22563: F, t7983: F, t173: F, t22583: F, t25694: F, t423: F) -> (F, F, F, F, F, F, F) {
    let t101150 = t5537 * t5546 * t3056;
    let t101173 = t5569 * t92652 * t6441;
    let t101193 = t37835 * t22582;
    let t101200 = t420 * t22535;
    let t101209 = t8042 * t92896;
    let t101228 = t7983 * t22563 * t3056;
    let t101243 = 0.49489226297715094073e-4 * t22583 * t173 * t423 * t25694;
    (t101150, t101173, t101193, t101200, t101209, t101228, t101243)
}
