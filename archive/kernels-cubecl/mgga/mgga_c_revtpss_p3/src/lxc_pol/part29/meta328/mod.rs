//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta328 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1239;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1240;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta328<F: Float>(t2461: F, t2471: F, t788: F, t9288: F, t787: F, t2453: F, t861: F, t2458: F, t2761: F, t786: F, t789: F, t212: F, t2760: F, t780: F, t689: F, t785: F, t860: F, t2439: F, t2772: F, t779: F, t781: F, t9292: F, t867: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11013, t11015, t11017, t11019, t11022, t11024) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1239::<F>(t2461, t2471, t788, t9288, t787, t2453, t861, t2458, t2761, t786, t789, t212, t2760);
        let (t11026, t11030, t11037, t11040, t11043) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1240::<F>(t11024, t780, t689, t785, t860, t2439, t2772, t779, t781, t9292, t861, t867);
    (t11013, t11015, t11017, t11019, t11022, t11026, t11030, t11037, t11040, t11043)
}
