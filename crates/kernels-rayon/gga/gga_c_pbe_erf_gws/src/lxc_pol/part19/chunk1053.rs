//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1053/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1053(t11835: f64, t2121: f64, t9119: f64, t11796: f64, t11798: f64, t11803: f64, t11810: f64, t11812: f64, t11816: f64, t11818: f64, t11820: f64, t11824: f64, t11829: f64, t11833: f64, t2253: f64, t2277: f64, t2312: f64, t2343: f64) -> (f64, f64) {
    let t11836 = t2121 * t11835;
    let t11838 = t9119 * t11836 / 48.0_f64;
    let t11839 = -t11796 - t2253 * t11798 / 384.0_f64 - t2277 * t11803 / 1536.0_f64 - t11810 + t11812 - t11816 - t11818 - 5.0_f64 / 192.0_f64 * t2343 * t11820 - t2312 * t11824 / 192.0_f64 + t2277 * t11829 / 384.0_f64 + t11833 + t11838;
    (t11838, t11839)
}
