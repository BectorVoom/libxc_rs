//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta531 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1910;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1911;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta531<F: Float>(t5: F, t28115: F, t28157: F, t117: F, t7239: F, t7898: F, t197: F, t530: F, t2013: F, t5627: F, t8996: F, t1310: F, t1453: F, t28050: F, t28053: F, t28058: F, t28060: F, t28062: F, t28065: F, t28069: F, t4248: F, t508: F, t649: F, t651: F, t7007: F, t7725: F, t7883: F, t7894: F) -> (F, F, F, F, F, F) {
        let (t28159, t28160, t28165, t28166, t28167) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1910::<F>(t5, t28115, t28157, t117, t7239, t7898, t197, t530, t2013);
        let (t28168, t28171) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1911::<F>(t5627, t8996, t28167, t1310, t1453, t28050, t28053, t28058, t28060, t28062, t28065, t28069, t28160, t28165, t4248, t508, t649, t651, t7007, t7725, t7883, t7894);
    (t28159, t28160, t28166, t28167, t28168, t28171)
}
