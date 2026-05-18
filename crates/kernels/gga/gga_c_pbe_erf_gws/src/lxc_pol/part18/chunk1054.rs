//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1054/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1054<F: Float>(t11835: F, t2121: F, t9119: F, t11796: F, t11798: F, t11803: F, t11810: F, t11812: F, t11816: F, t11818: F, t11820: F, t11824: F, t11829: F, t11833: F, t2253: F, t2277: F, t2312: F, t2343: F) -> (F, F) {
    let t11836 = t2121 * t11835;
    let t11838 = t9119 * t11836 / F::new(48.0);
    let t11839 = -t11796 - t2253 * t11798 / F::new(384.0) - t2277 * t11803 / F::new(1536.0) - t11810 + t11812 - t11816 - t11818 - F::new(5.0) / F::new(192.0) * t2343 * t11820 - t2312 * t11824 / F::new(192.0) + t2277 * t11829 / F::new(384.0) + t11833 + t11838;
    (t11838, t11839)
}
