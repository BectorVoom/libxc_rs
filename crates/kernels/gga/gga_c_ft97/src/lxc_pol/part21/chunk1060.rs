//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1060/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1060<F: Float>(t92: F, t92173: F, t25890: F, t93506: F, t1586: F, t25846: F, t1317: F, t26031: F, t376: F, t25987: F, t6501: F, t93503: F, t1570: F, t6454: F, t1557: F, t1882: F, t25957: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t101651 = t92173 * t92;
    let t101661 = t93506 * t25890;
    let t101662 = t101661 / 9.0;
    let t101678 = t1586 * t25846;
    let t101687 = t1317 * t376 * t26031;
    let t101688 = t101687 / 9.0;
    let t101689 = t93506 * t25987;
    let t101690 = t101689 / 54.0;
    let t101691 = t93503 * t6501;
    let t101693 = t6454 * t1570;
    let t101703 = t6454 * t1557;
    let t101708 = t1882 * t25957;
    (t101651, t101661, t101662, t101678, t101687, t101688, t101689, t101690, t101691, t101693, t101703, t101708)
}
