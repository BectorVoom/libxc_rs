//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 989/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk989<F: Float>(t10307: F, t10309: F, t10311: F, t10317: F, t10319: F, t10321: F, t10322: F, t10324: F, t10328: F, t10330: F, t10332: F, t10334: F, t10338: F, t10342: F, t10346: F, t7147: F) -> F {
    let t11197 = t10307 + t10309 + t10311 + t10317 - t10319 + t10321 + t10322 + t7147 + t10324 + t10328 + t10330 + t10332 - t10334 - t10338 + t10342 - t10346;
    t11197
}
