//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2856/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2856<F: Float>(t13100: F, t828: F, t12699: F, t3624: F, t12879: F, t3625: F, t3630: F, t1260: F, t12975: F, t1247: F, t1251: F, t42994: F) -> (F, F, F, F, F, F) {
    let t44225 = t828 * t13100;
    let t44230 = t12699 * t3624;
    let t44250 = t828 * t12879;
    let t44252 = t3625 * t44250 * t3630;
    let t44260 = t12975 * t1260;
    let t44264 = t1247 * t42994 * t1251;
    (t44225, t44230, t44250, t44252, t44260, t44264)
}
