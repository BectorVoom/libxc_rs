//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 947/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk947<F: Float>(t3116: F, t8874: F, t337: F, t3791: F, t814: F, t2121: F, t9119: F, t11796: F, t11798: F, t11803: F, t11810: F, t11812: F, t11816: F, t11818: F, t11820: F, t11824: F, t11829: F, t2253: F, t2277: F, t2312: F, t2343: F) -> (F, F, F) {
    let t11833 = t3116 * t8874 / 24.0;
    let t11835 = t337 * t3791 * t814;
    let t11836 = t2121 * t11835;
    let t11838 = t9119 * t11836 / 48.0;
    let t11839 = -t11796 - t2253 * t11798 / 384.0 - t2277 * t11803 / 1536.0 - t11810 + t11812 - t11816 - t11818 - 5.0 / 192.0 * t2343 * t11820 - t2312 * t11824 / 192.0 + t2277 * t11829 / 384.0 + t11833 + t11838;
    (t11833, t11838, t11839)
}
