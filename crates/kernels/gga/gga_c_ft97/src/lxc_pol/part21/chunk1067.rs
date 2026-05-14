//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1067/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1067<F: Float>(t101689: F, t101708: F, t101718: F, t101767: F, t101771: F, t101778: F, t101781: F, t101811: F, t101823: F, t101882: F, t101898: F, t6418: F, t94035: F, t1307: F, t3289: F, t1286: F, t1637: F, t6422: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t102203 = t101689 / 18.0;
    let t102209 = 2.0 / 9.0 * t101708;
    let t102212 = 2.0 / 9.0 * t101718;
    let t102226 = t101767 / 6.0;
    let t102228 = 4.0 / 3.0 * t101771;
    let t102230 = 4.0 / 3.0 * t101778;
    let t102231 = 4.0 / 3.0 * t101781;
    let t102239 = 4.0 * t101811;
    let t102243 = 2.0 / 3.0 * t101823;
    let t102258 = 2.0 / 3.0 * t101882;
    let t102261 = t101898 / 9.0;
    let t102270 = t94035 * t6418;
    let t102291 = t1307 * t3289;
    let t102312 = t1286 * t1637 * t6422;
    (t102203, t102209, t102212, t102226, t102228, t102230, t102231, t102239, t102243, t102258, t102261, t102270, t102291, t102312)
}
