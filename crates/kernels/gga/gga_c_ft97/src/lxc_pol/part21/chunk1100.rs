//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1100/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1100<F: Float>(t105543: F, t105559: F, t105567: F, t105637: F, t105671: F, t105677: F, t105685: F, t105696: F, t105740: F, t105743: F, t105760: F, t105765: F, t105770: F, t105772: F, t105809: F, t105815: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t106020 = 2.0 / 3.0 * t105543;
    let t106024 = 4.0 / 27.0 * t105559;
    let t106026 = 2.0 / 9.0 * t105567;
    let t106049 = 4.0 / 9.0 * t105637;
    let t106062 = t105671 / 18.0;
    let t106064 = 4.0 / 9.0 * t105677;
    let t106067 = 2.0 / 9.0 * t105685;
    let t106070 = t105696 / 18.0;
    let t106087 = t105740 / 18.0;
    let t106088 = 2.0 / 9.0 * t105743;
    let t106093 = 2.0 / 9.0 * t105760;
    let t106095 = 4.0 / 27.0 * t105765;
    let t106097 = 4.0 / 81.0 * t105770;
    let t106098 = 4.0 / 27.0 * t105772;
    let t106115 = t105809 / 54.0;
    let t106118 = t105815 / 9.0;
    (t106020, t106024, t106026, t106049, t106062, t106064, t106067, t106070, t106087, t106088, t106093, t106095, t106097, t106098, t106115, t106118)
}
