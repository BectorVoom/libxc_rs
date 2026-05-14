//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1101/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1101<F: Float>(t105826: F, t105846: F, t105848: F, t105862: F, t105894: F, t105941: F, t1349: F, t1637: F, t6588: F, t27253: F, t8392: F, t1384: F, t7763: F, t1391: F, t2101: F, t582: F, t5935: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t106121 = 2.0 / 27.0 * t105826;
    let t106127 = t105846 / 12.0;
    let t106128 = t105848 / 9.0;
    let t106133 = 4.0 / 3.0 * t105862;
    let t106144 = 4.0 / 9.0 * t105894;
    let t106160 = t105941 / 27.0;
    let t106200 = t1349 * t1637 * t6588;
    let t106214 = 4.0 / 27.0 * t8392 * t27253;
    let t106253 = t1384 * t7763;
    let t106296 = t2101 * t1391;
    let t106300 = t582 * t5935;
    (t106121, t106127, t106128, t106133, t106144, t106160, t106200, t106214, t106253, t106296, t106300)
}
