//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1315/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1315<F: Float>(t105570: F, t105598: F, t105617: F, t106049: F, t106062: F, t119906: F, t119913: F, t119917: F, t119922: F, t119926: F, t119930: F, t95177: F, t105711: F, t106064: F, t106067: F, t106070: F, t119935: F, t119938: F, t119943: F, t119948: F, t119953: F, t119955: F, t119959: F, t119963: F) -> (F, F) {
    let t120998 = -2.0 / 9.0 * t119906 + 16.0 / 27.0 * t105570 + 4.0 / 81.0 * t105598 - t119913 / 3.0 - t119917 / 3.0 + 8.0 / 27.0 * t105617 + 8.0 / 27.0 * t95177 - t106049 + t119922 / 9.0 + 4.0 / 9.0 * t119926 - 4.0 * t119930 - t106062;
    let t121009 = 2.0 / 3.0 * t119935 + t119938 / 9.0 - t106064 + t106067 - t119943 / 4.0 - t119948 / 8.0 + 5.0 / 16.0 * t119953 + t119955 / 27.0 - t106070 + 2.0 / 27.0 * t105711 - 2.0 / 9.0 * t119959 + t119963 / 12.0;
    (t120998, t121009)
}
