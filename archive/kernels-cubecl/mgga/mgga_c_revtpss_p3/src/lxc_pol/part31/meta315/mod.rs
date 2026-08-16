//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta315 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1315;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1316;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta315<F: Float>(t11003: F, t2439: F, t866: F, t225: F, t2461: F, t2471: F, t788: F, t9288: F, t787: F, t2453: F, t861: F, t2458: F, t785: F, t860: F, t780: F, t781: F, t9292: F, t867: F, t786: F, t2410: F, t261: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11004, t11006, t11007, t11008, t11013, t11015, t11017, t11019) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1315::<F>(t11003, t2439, t866, t225, t2461, t2471, t788, t9288, t787, t2453, t861, t2458);
        let (t11030, t11040, t11044, t11064) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1316::<F>(t785, t860, t780, t2439, t781, t9292, t861, t867, t786, t2410, t261);
    (t11004, t11006, t11007, t11008, t11013, t11015, t11017, t11019, t11030, t11040, t11044, t11064)
}
